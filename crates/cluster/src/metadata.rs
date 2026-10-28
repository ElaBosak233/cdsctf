//! Versioned metadata shared by creation, HTTP projection and resource cleanup.

use std::collections::BTreeMap;

use k8s_openapi::{api::core::v1::Pod, apimachinery::pkg::apis::meta::v1::ObjectMeta};
use serde::{Deserialize, Serialize};

use crate::traits::ClusterError;

pub const APP: &str = "cds/app";
pub const INSTANCE_ID: &str = "cds/instance_id";
pub const USER_ID: &str = "cds/user_id";
pub const TEAM_ID: &str = "cds/team_id";
pub const GAME_ID: &str = "cds/game_id";
pub const CHALLENGE_ID: &str = "cds/challenge_id";
pub const MODE: &str = "cds/mode";
pub const VERSION: &str = "cds/schema-version";
pub const DURATION: &str = "cds/duration";
pub const RENEW: &str = "cds/renew";
pub const RENEWAL_LIMIT: u32 = 3;
pub const RENEWAL_WINDOW_SECONDS: i64 = 600;
pub const MANAGED_SELECTOR: &str = "cds/app=challenges";

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceMode {
    Practice,
    Game,
    Debug,
}

/// `creator_user_id` is the creator; a team instance belongs to `team_id`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InstanceIdentity {
    pub id: String,
    pub creator_user_id: i64,
    pub challenge_id: i64,
    pub team_id: Option<i64>,
    pub game_id: Option<i64>,
    pub mode: InstanceMode,
}

pub fn invalid(field: &str) -> ClusterError {
    ClusterError::InvalidMetadata(field.to_owned())
}

pub fn validate_id(id: &str) -> Result<(), ClusterError> {
    if id.is_empty()
        || id.len() > 59
        || !id
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        || id.starts_with('-')
        || id.ends_with('-')
    {
        return Err(invalid(INSTANCE_ID));
    }
    Ok(())
}

pub fn selector(extra: &str) -> String {
    if extra.is_empty() {
        MANAGED_SELECTOR.to_owned()
    } else {
        format!("{MANAGED_SELECTOR},{extra}")
    }
}

impl InstanceIdentity {
    pub fn parse(metadata: &ObjectMeta) -> Result<Self, ClusterError> {
        let labels = metadata.labels.as_ref().ok_or_else(|| invalid(APP))?;
        if labels.get(APP).map(String::as_str) != Some("challenges") {
            return Err(invalid(APP));
        }
        let id = labels
            .get(INSTANCE_ID)
            .ok_or_else(|| invalid(INSTANCE_ID))?
            .clone();
        validate_id(&id)?;
        let positive = |key: &str| -> Result<i64, ClusterError> {
            labels
                .get(key)
                .and_then(|v| v.parse().ok())
                .filter(|v| *v > 0)
                .ok_or_else(|| invalid(key))
        };
        let optional = |key: &str| -> Result<Option<i64>, ClusterError> {
            match labels.get(key).map(String::as_str) {
                None | Some("0") => Ok(None), // Legacy non-game pods used zero sentinels.
                Some(value) => value
                    .parse::<i64>()
                    .ok()
                    .filter(|v| *v > 0)
                    .map(Some)
                    .ok_or_else(|| invalid(key)),
            }
        };
        let team_id = optional(TEAM_ID)?;
        let game_id = optional(GAME_ID)?;
        if team_id.is_some() != game_id.is_some() {
            return Err(invalid("game/team ownership"));
        }
        let mode = match labels.get(MODE).map(String::as_str) {
            Some("practice") => InstanceMode::Practice,
            Some("game") => InstanceMode::Game,
            Some("debug") => InstanceMode::Debug,
            None if game_id.is_some() => InstanceMode::Game,
            None => InstanceMode::Practice,
            _ => return Err(invalid(MODE)),
        };
        if (mode == InstanceMode::Game) != game_id.is_some() {
            return Err(invalid(MODE));
        }
        Ok(Self {
            id,
            creator_user_id: positive(USER_ID)?,
            challenge_id: positive(CHALLENGE_ID)?,
            team_id,
            game_id,
            mode,
        })
    }

    pub fn labels(&self, internet: bool) -> Result<BTreeMap<String, String>, ClusterError> {
        let mut labels = BTreeMap::from([
            (APP.to_owned(), "challenges".to_owned()),
            (
                "app.kubernetes.io/managed-by".to_owned(),
                "cdsctf".to_owned(),
            ),
            (INSTANCE_ID.to_owned(), self.id.clone()),
            (USER_ID.to_owned(), self.creator_user_id.to_string()),
            (CHALLENGE_ID.to_owned(), self.challenge_id.to_string()),
            ("cds/internet".to_owned(), internet.to_string()),
            (
                MODE.to_owned(),
                match self.mode {
                    InstanceMode::Practice => "practice",
                    InstanceMode::Game => "game",
                    InstanceMode::Debug => "debug",
                }
                .to_owned(),
            ),
        ]);
        if let Some(id) = self.team_id {
            labels.insert(TEAM_ID.to_owned(), id.to_string());
        }
        if let Some(id) = self.game_id {
            labels.insert(GAME_ID.to_owned(), id.to_string());
        }
        Self::parse(&ObjectMeta {
            labels: Some(labels.clone()),
            ..Default::default()
        })?;
        Ok(labels)
    }
}

#[derive(Clone, Debug)]
pub struct InstanceMetadata {
    pub identity: InstanceIdentity,
    pub created_at: i64,
    pub expires_at: i64,
    pub duration_seconds: i64,
    pub renewal_count: u32,
}

impl InstanceMetadata {
    pub fn annotations(duration_seconds: i64) -> Result<BTreeMap<String, String>, ClusterError> {
        if duration_seconds <= 0
            || duration_seconds
                .checked_mul(i64::from(RENEWAL_LIMIT) + 1)
                .is_none()
        {
            return Err(invalid(DURATION));
        }
        Ok(BTreeMap::from([
            (VERSION.to_owned(), "1".to_owned()),
            (DURATION.to_owned(), duration_seconds.to_string()),
            (RENEW.to_owned(), "0".to_owned()),
        ]))
    }

    pub fn parse(pod: &Pod) -> Result<Self, ClusterError> {
        let identity = InstanceIdentity::parse(&pod.metadata)?;
        let annotations = pod
            .metadata
            .annotations
            .as_ref()
            .ok_or_else(|| invalid(DURATION))?;
        match annotations.get(VERSION).map(String::as_str) {
            None | Some("1") => (),
            _ => return Err(invalid(VERSION)),
        }
        let duration_seconds = annotations
            .get(DURATION)
            .and_then(|v| v.parse::<i64>().ok())
            .filter(|v| *v > 0)
            .ok_or_else(|| invalid(DURATION))?;
        let renewal_count = annotations
            .get(RENEW)
            .and_then(|v| v.parse::<u32>().ok())
            .filter(|v| *v <= RENEWAL_LIMIT)
            .ok_or_else(|| invalid(RENEW))?;
        let created_at = pod
            .metadata
            .creation_timestamp
            .as_ref()
            .ok_or_else(|| invalid("creationTimestamp"))?
            .0
            .as_second();
        let expires_at = duration_seconds
            .checked_mul(i64::from(renewal_count) + 1)
            .and_then(|duration| created_at.checked_add(duration))
            .ok_or_else(|| invalid(DURATION))?;
        // Keep timestamps representable in the HTTP RFC 3339 projection.
        time::OffsetDateTime::from_unix_timestamp(created_at)
            .map_err(|_| invalid("creationTimestamp"))?;
        time::OffsetDateTime::from_unix_timestamp(expires_at).map_err(|_| invalid(DURATION))?;
        Ok(Self {
            identity,
            created_at,
            expires_at,
            duration_seconds,
            renewal_count,
        })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub fn pod() -> Pod {
        serde_json::from_value(serde_json::json!({
            "metadata": {
                "name": "cds-test", "uid": "pod-uid", "resourceVersion": "1",
                "creationTimestamp": "2026-01-01T00:00:00Z",
                "labels": { "cds/app": "challenges", "cds/instance_id": "test",
                    "cds/user_id": "1", "cds/challenge_id": "2", "cds/team_id": "0", "cds/game_id": "0" },
                "annotations": { "cds/duration": "1800", "cds/renew": "0" }
            },
            "spec": { "containers": [{ "name": "app", "image": "test" }] },
            "status": { "phase": "Pending" }
        })).unwrap()
    }

    #[test]
    fn legacy_metadata_and_new_metadata_have_the_same_deadline() {
        let legacy = pod();
        let old = InstanceMetadata::parse(&legacy).unwrap();
        assert_eq!(old.expires_at - old.created_at, 1800);
        assert_eq!(old.identity.mode, InstanceMode::Practice);
        assert_eq!(old.identity.team_id, None);
        let mut new = legacy;
        new.metadata.labels = Some(old.identity.labels(false).unwrap());
        new.metadata.annotations = Some(InstanceMetadata::annotations(1800).unwrap());
        let parsed = InstanceMetadata::parse(&new).unwrap();
        assert_eq!(parsed.identity, old.identity);
        assert_eq!(parsed.expires_at, old.expires_at);
        assert_eq!(new.metadata.annotations.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn malformed_metadata_is_never_guessed() {
        for (key, value) in [
            (DURATION, "0"),
            (DURATION, "-1"),
            (DURATION, "9223372036854775807"),
            (RENEW, "-1"),
            (RENEW, "4"),
            (RENEW, "abc"),
            (VERSION, "2"),
        ] {
            let mut pod = pod();
            pod.metadata
                .annotations
                .as_mut()
                .unwrap()
                .insert(key.to_owned(), value.to_owned());
            assert!(InstanceMetadata::parse(&pod).is_err(), "{key}={value}");
        }
        let mut pod = pod();
        pod.metadata.annotations.as_mut().unwrap().remove(RENEW);
        assert!(InstanceMetadata::parse(&pod).is_err());
        pod.metadata
            .labels
            .as_mut()
            .unwrap()
            .insert(TEAM_ID.to_owned(), "3".to_owned());
        assert!(InstanceIdentity::parse(&pod.metadata).is_err());
    }

    #[test]
    fn foreign_resources_and_selector_injection_are_rejected() {
        let mut pod = pod();
        pod.metadata.labels.as_mut().unwrap().remove(APP);
        assert!(InstanceIdentity::parse(&pod.metadata).is_err());
        for id in ["", "test,cds/user_id=2", "-test", "test/"] {
            assert!(validate_id(id).is_err());
        }
        assert_eq!(selector(""), MANAGED_SELECTOR);
    }
}
