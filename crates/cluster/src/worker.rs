use std::time;

use k8s_openapi::chrono;
use tracing::info;

pub async fn cleaner() {
    tokio::spawn(async {
        let interval = time::Duration::from_secs(10);
        loop {
            let pods = crate::get_pods_list().await.unwrap_or_default();
            for pod in pods {
                let id = pod
                    .metadata
                    .labels
                    .unwrap_or_default()
                    .get("cds/env_id")
                    .map(|s| s.to_owned())
                    .unwrap_or_default();

                let started_at = pod.metadata.creation_timestamp.unwrap().0.timestamp();

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

                let now = chrono::Utc::now().timestamp();

                if now > started_at + (renew + 1) * duration {
                    crate::delete_pod(&id).await.unwrap();
                    crate::delete_service(&id).await.unwrap();
                    info!("Cleaned up invalid cluster {0}", id);
                }
            }
            tokio::time::sleep(interval).await;
        }
    });
}
