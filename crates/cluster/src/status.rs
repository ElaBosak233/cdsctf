//! Pure, order-independent instance state projection. No cleanup decisions
//! here.

use k8s_openapi::api::core::v1::{ContainerStatus, Pod};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum InstanceState {
    Starting,
    Running,
    Unhealthy,
    Succeeded,
    Failed,
    Stopping,
    Unknown,
}

#[derive(
    Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, utoipa::ToSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum InstanceReason {
    Running,
    Scheduling,
    Initializing,
    PullingImage,
    Starting,
    NotReady,
    ImagePullFailed,
    InvalidConfiguration,
    CrashLoopBackOff,
    ContainerFailed,
    PodFailed,
    Completed,
    Stopping,
    InvalidMetadata,
    Unknown,
}

#[derive(Clone, Debug, Serialize, utoipa::ToSchema)]
pub struct InstanceObservation {
    pub state: InstanceState,
    pub reason: InstanceReason,
    pub ready: bool,
    pub ready_containers: usize,
    pub total_containers: usize,
}

impl InstanceState {
    pub fn is_terminal(self) -> bool {
        matches!(self, Self::Succeeded | Self::Failed)
    }

    /// Stopping, recovering and unknown pods still reserve a slot.
    pub fn counts_toward_quota(self) -> bool {
        !self.is_terminal()
    }

    pub fn can_renew(self) -> bool {
        matches!(self, Self::Starting | Self::Running | Self::Unhealthy)
    }

    /// Transitional projection for clients predating the typed `state` field.
    pub fn legacy_status(self) -> &'static str {
        match self {
            Self::Starting => "waiting",
            Self::Running => "running",
            Self::Succeeded | Self::Failed => "terminated",
            Self::Stopping => "stopping",
            Self::Unhealthy => "unhealthy",
            Self::Unknown => "unknown",
        }
    }
}

fn container_problem(status: &ContainerStatus) -> Option<InstanceReason> {
    let state = status.state.as_ref()?;
    if let Some(waiting) = &state.waiting {
        return match waiting.reason.as_deref() {
            Some("ErrImagePull" | "ImagePullBackOff" | "InvalidImageName") => {
                Some(InstanceReason::ImagePullFailed)
            }
            Some("CreateContainerConfigError" | "CreateContainerError" | "RunContainerError") => {
                Some(InstanceReason::InvalidConfiguration)
            }
            Some("CrashLoopBackOff") => Some(InstanceReason::CrashLoopBackOff),
            _ => None,
        };
    }
    state
        .terminated
        .as_ref()
        .filter(|s| s.exit_code != 0)
        .map(|_| InstanceReason::ContainerFailed)
}

pub fn observe(pod: &Pod) -> InstanceObservation {
    let spec = pod.spec.as_ref();
    let status = pod.status.as_ref();
    let containers = status
        .and_then(|s| s.container_statuses.as_deref())
        .unwrap_or_default();
    let init_statuses = status
        .and_then(|s| s.init_container_statuses.as_deref())
        .unwrap_or_default();
    let expected = spec.map(|s| s.containers.as_slice()).unwrap_or_default();
    let ready_containers = expected
        .iter()
        .filter(|container| {
            containers.iter().any(|s| {
                s.name == container.name
                    && s.ready
                    && s.state
                        .as_ref()
                        .is_some_and(|state| state.running.is_some())
            })
        })
        .count();
    let mut result = InstanceObservation {
        state: InstanceState::Unknown,
        reason: InstanceReason::Unknown,
        ready: false,
        ready_containers,
        total_containers: expected.len(),
    };
    let mut set = |state, reason| {
        result.state = state;
        result.reason = reason;
        result.clone()
    };
    if pod.metadata.deletion_timestamp.is_some() {
        return set(InstanceState::Stopping, InstanceReason::Stopping);
    }
    match status.and_then(|s| s.phase.as_deref()) {
        Some("Succeeded") => return set(InstanceState::Succeeded, InstanceReason::Completed),
        Some("Failed") => return set(InstanceState::Failed, InstanceReason::PodFailed),
        Some("Unknown") => return result,
        _ => (),
    }
    if expected.is_empty() {
        return result;
    }
    // Select a deterministic reason rather than letting the last container win.
    if let Some(reason) = containers
        .iter()
        .chain(init_statuses)
        .filter_map(container_problem)
        .min()
    {
        return set(InstanceState::Unhealthy, reason);
    }
    let conditions = status
        .and_then(|s| s.conditions.as_deref())
        .unwrap_or_default();
    let condition = |name: &str| {
        conditions
            .iter()
            .find(|c| c.type_ == name)
            .map(|c| c.status.as_str())
    };
    if condition("Ready") == Some("Unknown") {
        return result;
    }
    if condition("PodScheduled") == Some("False")
        || spec.is_some_and(|s| s.node_name.is_none()) && containers.is_empty()
    {
        return set(InstanceState::Starting, InstanceReason::Scheduling);
    }
    let initialized = spec
        .and_then(|s| s.init_containers.as_ref())
        .is_none_or(|init| {
            init.iter().all(|container| {
                init_statuses
                    .iter()
                    .find(|s| s.name == container.name)
                    .is_some_and(|s| {
                        if container.restart_policy.as_deref() == Some("Always") {
                            s.started == Some(true)
                        } else {
                            s.state
                                .as_ref()
                                .and_then(|state| state.terminated.as_ref())
                                .is_some_and(|t| t.exit_code == 0)
                        }
                    })
            })
        });
    if !initialized || condition("Initialized") == Some("False") {
        return set(InstanceState::Starting, InstanceReason::Initializing);
    }
    if status.and_then(|s| s.phase.as_deref()) == Some("Running")
        && ready_containers == expected.len()
        && condition("Ready") == Some("True")
    {
        result.state = InstanceState::Running;
        result.reason = InstanceReason::Running;
        result.ready = true;
        return result;
    }
    let reason = if containers.iter().any(|s| {
        s.state
            .as_ref()
            .and_then(|state| state.waiting.as_ref())
            .is_some_and(|s| s.reason.as_deref() == Some("ContainerCreating"))
    }) {
        InstanceReason::PullingImage
    } else if status.and_then(|s| s.phase.as_deref()) == Some("Running") {
        InstanceReason::NotReady
    } else {
        InstanceReason::Starting
    };
    set(InstanceState::Starting, reason)
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::*;
    use crate::metadata::tests::pod;

    fn with_status(status: Value, two_containers: bool) -> Pod {
        let mut value = serde_json::to_value(pod()).unwrap();
        value["spec"]["nodeName"] = json!("node-1");
        if two_containers {
            value["spec"]["containers"]
                .as_array_mut()
                .unwrap()
                .push(json!({"name":"other"}));
        }
        value["status"] = status;
        serde_json::from_value(value).unwrap()
    }

    #[test]
    fn a_healthy_container_cannot_hide_a_failing_container() {
        let status = json!({"phase":"Running", "conditions":[{"type":"Ready","status":"False"}],
        "containerStatuses":[
            {"name":"app","ready":false,"restartCount":2,"image":"test","imageID":"", "state":{"waiting":{"reason":"CrashLoopBackOff"}}},
            {"name":"other","ready":true,"restartCount":0,"image":"test","imageID":"", "state":{"running":{}}}
        ]});
        let mut pod = with_status(status, true);
        let first = observe(&pod);
        pod.status
            .as_mut()
            .unwrap()
            .container_statuses
            .as_mut()
            .unwrap()
            .reverse();
        let second = observe(&pod);
        assert_eq!(first.state, InstanceState::Unhealthy);
        assert_eq!(first.reason, InstanceReason::CrashLoopBackOff);
        assert_eq!(first.state, second.state);
        assert_eq!(first.reason, second.reason);
        assert!(!first.ready);
        assert_eq!(first.ready_containers, 1);
    }

    #[test]
    fn running_requires_all_expected_containers_and_pod_readiness() {
        let status = json!({"phase":"Running","conditions":[{"type":"Ready","status":"True"}],
            "containerStatuses":[{"name":"app","ready":true,"restartCount":0,"image":"test","imageID":"","state":{"running":{}}}]});
        let mut pod = with_status(status.clone(), false);
        assert!(observe(&pod).ready);
        pod.status.as_mut().unwrap().conditions.as_mut().unwrap()[0].status = "False".into();
        assert_eq!(observe(&pod).state, InstanceState::Starting);
        assert!(!observe(&with_status(status, true)).ready);
    }

    #[test]
    fn lifecycle_states_take_precedence_and_missing_status_is_explicit() {
        for (phase, expected) in [
            ("Succeeded", InstanceState::Succeeded),
            ("Failed", InstanceState::Failed),
            ("Unknown", InstanceState::Unknown),
        ] {
            let mut pod = with_status(json!({"phase":phase}), false);
            assert_eq!(observe(&pod).state, expected);
            pod.metadata.deletion_timestamp = pod.metadata.creation_timestamp.clone();
            assert_eq!(observe(&pod).state, InstanceState::Stopping);
        }
        assert_eq!(observe(&pod()).reason, InstanceReason::Scheduling);
        assert!(InstanceState::Unknown.counts_toward_quota());
        assert!(InstanceState::Stopping.counts_toward_quota());
        assert!(!InstanceState::Failed.counts_toward_quota());
    }

    #[test]
    fn init_failures_and_image_failures_can_recover() {
        for (raw, reason) in [
            ("ImagePullBackOff", InstanceReason::ImagePullFailed),
            (
                "CreateContainerConfigError",
                InstanceReason::InvalidConfiguration,
            ),
        ] {
            let status = json!({"phase":"Pending", "initContainerStatuses":[{"name":"init","image":"test","imageID":"","ready":false,"restartCount":0,"state":{"waiting":{"reason":raw}}}]});
            let pod = with_status(status, false);
            assert_eq!(observe(&pod).reason, reason);
            assert_eq!(observe(&pod).state, InstanceState::Unhealthy);
            assert!(observe(&pod).state.can_renew());
        }
    }
}
