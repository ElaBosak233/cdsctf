use cds_cluster::k8s_openapi::api::core::v1::Pod;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Env {
    pub id: String,
    pub user_id: i64,
    pub team_id: i64,
    pub game_id: i64,
    pub challenge_id: Uuid,

    pub ports: Vec<i32>,
    pub public_entry: Option<String>,
    pub nats: Option<String>,

    pub status: String,
    pub reason: String,

    pub renew: i64,
    pub duration: i64,
    pub started_at: i64,
}

impl From<Pod> for Env {
    fn from(pod: Pod) -> Self {
        let labels = pod.metadata.labels.unwrap_or_default();

        let id = labels
            .get("cds/env_id")
            .map(|s| s.to_owned())
            .unwrap_or_default()
            .to_owned();
        let user_id = labels
            .get("cds/user_id")
            .map(|s| s.to_owned())
            .unwrap_or_default()
            .to_owned()
            .parse::<i64>()
            .unwrap_or(0);
        let team_id = labels
            .get("cds/team_id")
            .map(|s| s.to_owned())
            .unwrap_or_default()
            .to_owned()
            .parse::<i64>()
            .unwrap_or(0);
        let game_id = labels
            .get("cds/game_id")
            .map(|s| s.to_owned())
            .unwrap_or_default()
            .to_owned()
            .parse::<i64>()
            .unwrap_or(0);
        let challenge_id = Uuid::parse_str(
            &labels
                .get("cds/challenge_id")
                .map(|s| s.to_owned())
                .unwrap_or_default()
                .to_owned(),
        )
        .unwrap_or_default();

        let annotations = pod.metadata.annotations.unwrap_or_default();

        let ports = serde_json::from_str::<Vec<i32>>(
            &annotations
                .get("cds/ports")
                .map(|s| s.to_owned())
                .unwrap_or_default(),
        )
        .unwrap_or_default();
        let nats = annotations.get("cds/nats").map(|s| s.to_owned()).to_owned();
        let duration = annotations
            .get("cds/duration")
            .map(|s| s.to_owned())
            .unwrap_or_default()
            .to_owned()
            .parse::<i64>()
            .unwrap_or(0);
        let renew = annotations
            .get("cds/renew")
            .map(|s| s.to_owned())
            .unwrap_or_default()
            .to_owned()
            .parse::<i64>()
            .unwrap_or(0);

        let mut status = "".to_owned();
        let mut reason = "".to_owned();

        let _ = pod
            .status
            .unwrap_or_default()
            .container_statuses
            .unwrap_or_default()
            .iter()
            .for_each(|s| {
                let container_state = s.to_owned().state.unwrap_or_default();
                if let Some(waiting) = container_state.waiting {
                    status = "waiting".to_owned();
                    if let Some(r) = waiting.reason {
                        reason = r.clone();
                    }
                }
                if let Some(_) = container_state.running {
                    status = "running".to_owned();
                }
                if let Some(terminated) = container_state.terminated {
                    status = "terminated".to_owned();
                    if let Some(r) = terminated.reason {
                        reason = r.clone();
                    }
                }
            });

        let started_at = pod.metadata.creation_timestamp.unwrap().0.timestamp();

        let node_name = pod.spec.unwrap_or_default().node_name.unwrap_or_default();

        let public_entry = cds_config::get_constant()
            .cluster
            .public_entries
            .get(&node_name)
            .cloned();

        Env {
            id,
            user_id,
            team_id,
            game_id,
            challenge_id,
            public_entry,
            ports,
            nats,
            status,
            reason,
            renew,
            duration,
            started_at,
        }
    }
}
