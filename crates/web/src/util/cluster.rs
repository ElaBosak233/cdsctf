//! Public instance projection. Kubernetes diagnostics remain on the server.

use cds_cluster::{
    k8s_openapi::api::core::v1::{Pod, Service},
    lifecycle::{check_renewal, now},
    metadata::{INSTANCE_ID, InstanceIdentity, InstanceMetadata, InstanceMode, RENEWAL_LIMIT},
    status::{self, InstanceReason, InstanceState},
    traits::{ClusterError, Nat},
};
use cds_db::challenge::Port;
use cds_env::Env;
use serde::Serialize;

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct Instance {
    pub id: String,
    pub user_id: i64,
    pub team_id: i64,
    pub game_id: i64,
    pub challenge_id: i64,
    pub mode: InstanceMode,
    pub ports: Vec<Port>,
    pub public_entry: Option<String>,
    pub nats: Vec<Nat>,
    pub state: InstanceState,
    pub reason_code: InstanceReason,
    pub ready: bool,
    pub ready_containers: usize,
    pub total_containers: usize,
    #[serde(with = "time::serde::rfc3339")]
    pub created_at: time::OffsetDateTime,
    #[serde(with = "time::serde::rfc3339::option")]
    pub expires_at: Option<time::OffsetDateTime>,
    pub renewal_count: Option<u32>,
    pub renewal_limit: u32,
    pub can_renew: bool,
    pub can_stop: bool,
    /// Compatibility fields. New clients use state/reason_code and expires_at.
    pub status: String,
    pub reason: String,
    pub renew: i64,
    pub duration: i64,
    #[serde(with = "time::serde::rfc3339")]
    pub started_at: time::OffsetDateTime,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct StopInstanceResponse {
    pub state: InstanceState,
}

impl Default for StopInstanceResponse {
    fn default() -> Self {
        Self {
            state: InstanceState::Stopping,
        }
    }
}

impl Instance {
    pub fn from_pod(
        pod: &Pod,
        service: Option<&Service>,
        env: &Env,
        at: i64,
    ) -> Result<Self, ClusterError> {
        let identity = InstanceIdentity::parse(&pod.metadata)?;
        let created_at = pod
            .metadata
            .creation_timestamp
            .as_ref()
            .and_then(|t| time::OffsetDateTime::from_unix_timestamp(t.0.as_second()).ok())
            .ok_or_else(|| cds_cluster::metadata::invalid("creationTimestamp"))?;
        let mut observation = status::observe(pod);
        let lifecycle = match InstanceMetadata::parse(pod) {
            Ok(metadata) => Some(metadata),
            Err(error) => {
                tracing::warn!(instance_id = %identity.id, error = ?error, "invalid instance lifecycle metadata");
                if observation.state != InstanceState::Stopping {
                    observation.state = InstanceState::Unknown;
                    observation.reason = InstanceReason::InvalidMetadata;
                }
                observation.ready = false;
                None
            }
        };
        let can_renew = lifecycle
            .as_ref()
            .is_some_and(|metadata| check_renewal(pod, metadata, at).is_ok());
        let expires_at = lifecycle
            .as_ref()
            .and_then(|m| time::OffsetDateTime::from_unix_timestamp(m.expires_at).ok());
        let ports = pod
            .spec
            .as_ref()
            .into_iter()
            .flat_map(|s| &s.containers)
            .flat_map(|container| container.ports.as_deref().unwrap_or_default())
            .map(|port| Port {
                port: port.container_port,
                protocol: port.protocol.clone().unwrap_or_else(|| "TCP".into()),
            })
            .collect::<Vec<_>>();
        let nats = service
            .and_then(|service| service.spec.as_ref())
            .and_then(|spec| spec.ports.as_ref())
            .into_iter()
            .flatten()
            .filter_map(|port| {
                Some(Nat {
                    port: port.port,
                    node_port: port.node_port?,
                    protocol: port.protocol.clone().unwrap_or_else(|| "TCP".into()),
                })
            })
            .collect();
        let mut unique_ports = Vec::new();
        for port in ports {
            if !unique_ports
                .iter()
                .any(|p: &Port| p.port == port.port && p.protocol == port.protocol)
            {
                unique_ports.push(port);
            }
        }
        Ok(Self {
            id: identity.id,
            user_id: identity.creator_user_id,
            team_id: identity.team_id.unwrap_or(0),
            game_id: identity.game_id.unwrap_or(0),
            challenge_id: identity.challenge_id,
            mode: identity.mode,
            ports: unique_ports,
            public_entry: Some(env.cluster.public_entry.clone()),
            nats,
            state: observation.state,
            reason_code: observation.reason,
            ready: observation.ready,
            ready_containers: observation.ready_containers,
            total_containers: observation.total_containers,
            created_at,
            expires_at,
            renewal_count: lifecycle.as_ref().map(|m| m.renewal_count),
            renewal_limit: RENEWAL_LIMIT,
            can_renew,
            can_stop: observation.state != InstanceState::Stopping,
            status: observation.state.legacy_status().to_owned(),
            // Old clients automatically delete waiting states with other reasons.
            reason: if observation.state == InstanceState::Starting {
                "ContainerCreating".into()
            } else {
                String::new()
            },
            renew: lifecycle.as_ref().map_or(0, |m| i64::from(m.renewal_count)),
            duration: lifecycle.as_ref().map_or(0, |m| m.duration_seconds),
            started_at: created_at,
        })
    }
}

/// Join only matching service metadata; do not trust stale annotation
/// snapshots.
pub fn project_instances(pods: &[Pod], services: &[Service], env: &Env) -> Vec<Instance> {
    let at = now();
    let mut instances = pods.iter().filter_map(|pod| {
        let service = pod.metadata.labels.as_ref().and_then(|labels| labels.get(INSTANCE_ID)).and_then(|id| {
            services.iter().find(|s| s.metadata.labels.as_ref().and_then(|labels| labels.get(INSTANCE_ID)) == Some(id))
        });
        match Instance::from_pod(pod, service, env, at) {
            Ok(instance) => Some(instance),
            Err(error) => {
                tracing::warn!(pod = ?pod.metadata.name, error = ?error, "skipping invalid instance identity");
                None
            }
        }
    }).collect::<Vec<_>>();
    // A retained terminal pod must not hide a newly created instance of the
    // same challenge.
    instances.sort_by(|a, b| {
        a.state
            .is_terminal()
            .cmp(&b.state.is_terminal())
            .then_with(|| b.created_at.cmp(&a.created_at))
            .then_with(|| a.id.cmp(&b.id))
    });
    instances
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    fn pod() -> Pod {
        serde_json::from_value(json!({
            "metadata":{"name":"cds-test","creationTimestamp":"2026-01-01T00:00:00Z",
                "labels":{"cds/app":"challenges","cds/instance_id":"test","cds/user_id":"1","cds/challenge_id":"2"},
                "annotations":{"cds/duration":"1800","cds/renew":"0","cds/nats":"invalid"}},
            "spec":{"containers":[{"name":"app","ports":[{"containerPort":80,"protocol":"TCP"}]}]},
            "status":{"phase":"Pending"}
        })).unwrap()
    }

    #[test]
    fn projection_uses_resource_ports_and_keeps_invalid_lifecycle_visible() {
        let mut pod = pod();
        let service = serde_json::from_value(
            json!({"spec":{"ports":[{"port":80,"nodePort":30001,"protocol":"TCP"}]}}),
        )
        .unwrap();
        let env = Env::default();
        let view = Instance::from_pod(&pod, Some(&service), &env, 0).unwrap();
        assert_eq!(view.ports[0].port, 80);
        assert_eq!(view.nats[0].node_port, 30001);
        assert_eq!(view.state, InstanceState::Starting);
        assert_eq!(view.reason, "ContainerCreating");
        pod.metadata
            .annotations
            .as_mut()
            .unwrap()
            .remove("cds/renew");
        let view = Instance::from_pod(&pod, None, &env, 0).unwrap();
        assert_eq!(view.state, InstanceState::Unknown);
        assert_eq!(view.reason_code, InstanceReason::InvalidMetadata);
        assert!(view.expires_at.is_none());
        assert!(!view.can_renew);
        assert!(view.can_stop);
    }

    #[test]
    fn retained_terminal_instances_do_not_hide_active_instances() {
        let mut terminal = pod();
        terminal.status.as_mut().unwrap().phase = Some("Failed".into());
        let mut active = pod();
        active
            .metadata
            .labels
            .as_mut()
            .unwrap()
            .insert(INSTANCE_ID.into(), "active".into());
        let views = project_instances(&[terminal, active], &[], &Env::default());
        assert_eq!(views[0].id, "active");
        assert_eq!(views[1].state, InstanceState::Failed);
    }
}
