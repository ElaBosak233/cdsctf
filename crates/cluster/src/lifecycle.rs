//! Lifecycle rules and conditional Kubernetes writes.

use k8s_openapi::api::core::v1::Pod;
use kube::{
    Api,
    api::{DeleteParams, Patch, PatchParams, Preconditions},
};
use serde_json::json;

use crate::{
    metadata::{self, InstanceIdentity, InstanceMetadata, RENEWAL_LIMIT, RENEWAL_WINDOW_SECONDS},
    status::observe,
    traits::ClusterError,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RenewalError {
    #[error("no_more_renewal")]
    LimitReached,
    #[error("renewal_within_10_minutes")]
    TooEarly,
    #[error("instance_expired")]
    Expired,
    #[error("instance_not_active")]
    NotActive,
}

pub fn check_renewal(pod: &Pod, metadata: &InstanceMetadata, now: i64) -> Result<(), RenewalError> {
    if !observe(pod).state.can_renew() {
        return Err(RenewalError::NotActive);
    }
    if metadata.expires_at <= now {
        return Err(RenewalError::Expired);
    }
    if metadata.renewal_count >= RENEWAL_LIMIT {
        return Err(RenewalError::LimitReached);
    }
    if metadata.expires_at.saturating_sub(now) > RENEWAL_WINDOW_SECONDS {
        return Err(RenewalError::TooEarly);
    }
    Ok(())
}

pub fn now() -> i64 {
    time::OffsetDateTime::now_utc().unix_timestamp()
}

fn name(pod: &Pod) -> Result<&str, ClusterError> {
    pod.metadata
        .name
        .as_deref()
        .ok_or_else(|| metadata::invalid("name"))
}

fn preconditions(pod: &Pod, check_version: bool) -> Result<Preconditions, ClusterError> {
    let uid = pod
        .metadata
        .uid
        .clone()
        .ok_or_else(|| metadata::invalid("uid"))?;
    let resource_version = if check_version {
        Some(
            pod.metadata
                .resource_version
                .clone()
                .ok_or_else(|| metadata::invalid("resourceVersion"))?,
        )
    } else {
        None
    };
    Ok(Preconditions {
        uid: Some(uid),
        resource_version,
    })
}

/// Revalidate the authorized identity and every lifecycle rule after conflicts.
pub async fn renew(api: &Api<Pod>, authorized: &Pod) -> Result<Pod, ClusterError> {
    let identity = InstanceIdentity::parse(&authorized.metadata)?;
    let uid = authorized
        .metadata
        .uid
        .as_ref()
        .ok_or_else(|| metadata::invalid("uid"))?;
    let pod_name = name(authorized)?;
    for _ in 0..3 {
        let pod = api.get(pod_name).await?;
        let metadata = InstanceMetadata::parse(&pod)?;
        if metadata.identity != identity || pod.metadata.uid.as_ref() != Some(uid) {
            return Err(ClusterError::Conflict);
        }
        check_renewal(&pod, &metadata, now())?;
        let version = preconditions(&pod, true)?.resource_version;
        let patch = json!({"metadata": {
            "resourceVersion": version,
            "annotations": { metadata::RENEW: (metadata.renewal_count + 1).to_string() }
        }});
        match api
            .patch(pod_name, &PatchParams::default(), &Patch::Merge(patch))
            .await
        {
            Ok(pod) => return Ok(pod),
            Err(kube::Error::Api(error)) if error.code == 409 => continue,
            Err(error) => return Err(error.into()),
        }
    }
    Err(ClusterError::Conflict)
}

/// Manual stop is UID-conditional but does not race with an ordinary renewal.
pub async fn stop(api: &Api<Pod>, pod: &Pod) -> Result<(), ClusterError> {
    InstanceIdentity::parse(&pod.metadata)?;
    let params = DeleteParams {
        preconditions: Some(preconditions(pod, false)?),
        ..Default::default()
    };
    match api.delete(name(pod)?, &params).await {
        Ok(_) => Ok(()),
        Err(kube::Error::Api(error)) if error.code == 404 => Ok(()),
        Err(error) => Err(error.into()),
    }
}

/// A renewed or otherwise changed pod must survive a stale cleaner snapshot.
pub async fn delete_if_expired(api: &Api<Pod>, pod: &Pod, now: i64) -> Result<bool, ClusterError> {
    let metadata = InstanceMetadata::parse(pod)?;
    if metadata.expires_at > now {
        return Ok(false);
    }
    let params = DeleteParams {
        preconditions: Some(preconditions(pod, true)?),
        ..Default::default()
    };
    match api.delete(name(pod)?, &params).await {
        Ok(_) => Ok(true),
        Err(kube::Error::Api(error)) if error.code == 404 => Ok(true),
        Err(kube::Error::Api(error)) if error.code == 409 => Ok(false),
        Err(error) => Err(error.into()),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use axum::{
        Json, Router,
        extract::{Request, State},
        http::{Method, StatusCode},
    };
    use serde_json::{Value, json};

    use super::*;
    use crate::metadata::tests::pod;

    #[test]
    fn renewal_boundaries_and_terminal_states() {
        let mut pod = pod();
        let mut metadata = InstanceMetadata::parse(&pod).unwrap();
        assert_eq!(
            check_renewal(&pod, &metadata, metadata.expires_at - 601),
            Err(RenewalError::TooEarly)
        );
        assert!(check_renewal(&pod, &metadata, metadata.expires_at - 600).is_ok());
        assert_eq!(
            check_renewal(&pod, &metadata, metadata.expires_at),
            Err(RenewalError::Expired)
        );
        metadata.renewal_count = RENEWAL_LIMIT;
        assert_eq!(
            check_renewal(&pod, &metadata, metadata.expires_at - 1),
            Err(RenewalError::LimitReached)
        );
        pod.metadata.deletion_timestamp = pod.metadata.creation_timestamp.clone();
        assert_eq!(
            check_renewal(&pod, &metadata, metadata.expires_at - 1),
            Err(RenewalError::NotActive)
        );
    }

    #[derive(Default)]
    struct MockState {
        pod: Pod,
        requests: Vec<(Method, Value)>,
        patch_conflict: bool,
        delete_conflict: bool,
    }

    fn mock(state: MockState) -> (Api<Pod>, Arc<Mutex<MockState>>) {
        let state = Arc::new(Mutex::new(state));
        let app = Router::new().fallback(async |State(state): State<Arc<Mutex<MockState>>>, request: Request| {
            let method = request.method().clone();
            let bytes = axum::body::to_bytes(request.into_body(), 1024 * 1024).await.unwrap();
            let body = serde_json::from_slice::<Value>(&bytes).unwrap_or(Value::Null);
            let mut state = state.lock().unwrap();
            state.requests.push((method.clone(), body.clone()));
            if method == Method::PATCH && state.patch_conflict {
                state.patch_conflict = false;
                // Another renewal won. The retry must revalidate the window.
                state.pod.metadata.annotations.as_mut().unwrap().insert(metadata::RENEW.into(), "1".into());
                state.pod.metadata.resource_version = Some("2".into());
                return (StatusCode::CONFLICT, Json(json!({"apiVersion":"v1","kind":"Status","status":"Failure","reason":"Conflict","code":409,"message":"conflict"})));
            }
            if method == Method::DELETE {
                if state.delete_conflict {
                    return (StatusCode::CONFLICT, Json(json!({"apiVersion":"v1","kind":"Status","status":"Failure","reason":"Conflict","code":409,"message":"renewed"})));
                }
                return (StatusCode::OK, Json(json!({"apiVersion":"v1","kind":"Status","status":"Success","code":200})));
            }
            if method == Method::PATCH {
                state.pod.metadata.annotations.as_mut().unwrap().insert(metadata::RENEW.into(), body["metadata"]["annotations"][metadata::RENEW].as_str().unwrap().into());
            }
            let mut value = serde_json::to_value(&state.pod).unwrap();
            value["apiVersion"] = json!("v1"); value["kind"] = json!("Pod");
            (StatusCode::OK, Json(value))
        }).with_state(state.clone());
        (
            Api::namespaced(kube::Client::new(app, "test"), "test"),
            state,
        )
    }

    fn renewable_pod() -> Pod {
        let mut pod = pod();
        let timestamp = time::OffsetDateTime::from_unix_timestamp(now() - 1500)
            .unwrap()
            .format(&time::format_description::well_known::Rfc3339)
            .unwrap();
        let mut value = serde_json::to_value(&pod).unwrap();
        value["metadata"]["creationTimestamp"] = json!(timestamp);
        pod = serde_json::from_value(value).unwrap();
        pod
    }

    #[tokio::test]
    async fn renewal_only_patches_counter_and_rechecks_after_conflict() {
        let pod = renewable_pod();
        let (api, state) = mock(MockState {
            pod: pod.clone(),
            ..Default::default()
        });
        renew(&api, &pod).await.unwrap();
        let request = state
            .lock()
            .unwrap()
            .requests
            .iter()
            .find(|(method, _)| *method == Method::PATCH)
            .unwrap()
            .1
            .clone();
        assert_eq!(
            request,
            json!({"metadata":{"resourceVersion":"1","annotations":{"cds/renew":"1"}}})
        );
        let (api, state) = mock(MockState {
            pod: pod.clone(),
            patch_conflict: true,
            ..Default::default()
        });
        assert!(matches!(
            renew(&api, &pod).await,
            Err(ClusterError::Renewal(RenewalError::TooEarly))
        ));
        assert_eq!(
            state
                .lock()
                .unwrap()
                .requests
                .iter()
                .filter(|(method, _)| *method == Method::PATCH)
                .count(),
            1
        );
    }

    #[tokio::test]
    async fn stale_cleanup_uses_uid_and_version_and_does_not_win_over_renewal() {
        let pod = pod();
        let metadata = InstanceMetadata::parse(&pod).unwrap();
        let (api, state) = mock(MockState {
            pod: pod.clone(),
            delete_conflict: true,
            ..Default::default()
        });
        assert!(
            !delete_if_expired(&api, &pod, metadata.expires_at)
                .await
                .unwrap()
        );
        assert_eq!(
            state.lock().unwrap().requests[0].1["preconditions"],
            json!({"uid":"pod-uid","resourceVersion":"1"})
        );
        let (api, state) = mock(MockState {
            pod: pod.clone(),
            ..Default::default()
        });
        assert!(
            !delete_if_expired(&api, &pod, metadata.expires_at - 1)
                .await
                .unwrap()
        );
        assert!(state.lock().unwrap().requests.is_empty());
        let mut invalid = pod;
        invalid
            .metadata
            .annotations
            .as_mut()
            .unwrap()
            .remove(metadata::RENEW);
        assert!(
            delete_if_expired(&api, &invalid, metadata.expires_at)
                .await
                .is_err()
        );
        assert!(state.lock().unwrap().requests.is_empty());
    }

    #[tokio::test]
    async fn manual_stop_is_uid_conditional_and_renewal_rejects_replacement() {
        let pod = renewable_pod();
        let (api, state) = mock(MockState {
            pod: pod.clone(),
            ..Default::default()
        });
        stop(&api, &pod).await.unwrap();
        let body = state.lock().unwrap().requests[0].1.clone();
        assert_eq!(body["preconditions"], json!({"uid":"pod-uid"}));
        assert_ne!(body["gracePeriodSeconds"], json!(0));

        let mut replacement = pod.clone();
        replacement.metadata.uid = Some("replacement-uid".into());
        let (api, state) = mock(MockState {
            pod: replacement,
            ..Default::default()
        });
        assert!(matches!(
            renew(&api, &pod).await,
            Err(ClusterError::Conflict)
        ));
        assert!(
            state
                .lock()
                .unwrap()
                .requests
                .iter()
                .all(|(method, _)| *method == Method::GET)
        );
    }
}
