use cds_db::sea_orm::sqlx::types::time;
use tracing::info;

use crate::Cluster;

pub async fn cleaner(cluster: Cluster) {
    tokio::spawn(async move {
        let interval = std::time::Duration::from_secs(10);
        loop {
            let pods = cluster.get_pods_list().await.unwrap_or_default();
            for pod in pods {
                let id = pod
                    .metadata
                    .labels
                    .unwrap_or_default()
                    .get("cds/env_id")
                    .map(|s| s.to_owned())
                    .unwrap_or_default();

                // SAFETY: the creation_timestamp could be safely unwrapped.
                let started_at = pod.metadata.creation_timestamp.unwrap().0.as_second();

                let annotations = pod.metadata.annotations.unwrap_or_default();

                let renew = annotations
                    .get("cds/renew")
                    .map(|s| s.to_owned())
                    .unwrap_or_default()
                    .parse::<i64>()
                    .unwrap_or(3);
                let duration = annotations
                    .get("cds/duration")
                    .map(|s| s.to_owned())
                    .unwrap_or_default()
                    .parse::<i64>()
                    .unwrap_or(0);

                let now = time::OffsetDateTime::now_utc().unix_timestamp();

                if now > started_at + (renew + 1) * duration {
                    cluster.delete_pod(&id).await.unwrap();
                    cluster.delete_service(&id).await.unwrap();
                    info!("Cleaned up invalid cluster {0}", id);
                }
            }
            tokio::time::sleep(interval).await;
        }
    });
}
