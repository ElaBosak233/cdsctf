use std::time;

use cds_db::get_db;
use futures::StreamExt;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter};
use sea_orm::ActiveValue::{Set, Unchanged};
use tracing::info;
use uuid::Uuid;

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
                cds_cluster::delete(pod.id).await;
                cds_db::entity::pod::Entity::delete_by_id(pod.id)
                    .exec(get_db())
                    .await
                    .unwrap();
                info!("Cleaned up expired cluster: {0}", pod.id);
            }
            tokio::time::sleep(interval).await;
        }
    });
}
