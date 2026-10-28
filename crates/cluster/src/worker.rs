//! Conservative cleanup: malformed resources are logged, never guessed at.

use tracing::{info, warn};

use crate::{
    Cluster, lifecycle,
    metadata::{InstanceIdentity, InstanceMetadata},
};

pub async fn cleaner(cluster: Cluster) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = cleanup(&cluster).await {
                warn!(error = ?error, "instance cleanup scan failed");
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });
}

async fn cleanup(cluster: &Cluster) -> Result<(), crate::traits::ClusterError> {
    let pods = cluster.get_pods_list().await?;
    let now = lifecycle::now();
    for pod in &pods {
        let metadata = match InstanceMetadata::parse(pod) {
            Ok(metadata) => metadata,
            Err(error) => {
                warn!(pod = ?pod.metadata.name, error = ?error, "skipping instance with invalid metadata");
                continue;
            }
        };
        if metadata.expires_at <= now {
            match cluster.delete_expired_instance(pod, now).await {
                Ok(true) => {
                    info!(instance_id = %metadata.identity.id, "expired instance cleanup requested")
                }
                Ok(false) => (), // A newer version won, e.g. a concurrent renewal.
                Err(error) => {
                    warn!(instance_id = %metadata.identity.id, error = ?error, "expired instance cleanup failed")
                }
            }
        }
    }
    // Retry service cleanup even when the pod is already gone. Allow
    // provisioning a grace period and recheck the live API before removing
    // an apparent orphan.
    for service in cluster.get_services_list().await? {
        let Ok(identity) = InstanceIdentity::parse(&service.metadata) else {
            continue;
        };
        if pods.iter().any(|p| {
            p.metadata
                .labels
                .as_ref()
                .and_then(|l| l.get(crate::metadata::INSTANCE_ID))
                == Some(&identity.id)
        }) {
            continue;
        }
        if service
            .metadata
            .creation_timestamp
            .as_ref()
            .is_none_or(|t| now.saturating_sub(t.0.as_second()) < 60)
        {
            continue;
        }
        match cluster.get_pod(&identity.id).await {
            Err(crate::traits::ClusterError::NotFound(_)) => {
                cluster.delete_service(&identity.id).await?
            }
            Err(error) => {
                warn!(instance_id = %identity.id, error = ?error, "could not verify orphan service")
            }
            Ok(_) => (),
        }
    }
    Ok(())
}
