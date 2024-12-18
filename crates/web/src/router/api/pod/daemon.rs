use std::time;

use cds_db::get_db;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::info;

pub async fn init() {
    tokio::spawn(async {
        let interval = time::Duration::from_secs(10);
        loop {
            let pods = cds_db::entity::pod::Entity::find()
                .filter(cds_db::entity::pod::Column::RemovedAt.lte(chrono::Utc::now().timestamp()))
                .all(get_db())
                .await
                .unwrap();
            for pod in pods {
                cds_cluster::delete(pod.name.clone()).await;
                cds_db::entity::pod::Entity::delete_by_id(pod.id)
                    .exec(get_db())
                    .await
                    .unwrap();
                info!("Cleaned up expired cluster: {0}", pod.name);
            }
            tokio::time::sleep(interval).await;
        }
    });
}
