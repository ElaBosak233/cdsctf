use std::time;

use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tracing::info;

use crate::{cluster, database::get_db};

pub async fn init() {
    tokio::spawn(async {
        let interval = time::Duration::from_secs(10);
        loop {
            let pods = crate::model::pod::Entity::find()
                .filter(crate::model::pod::Column::RemovedAt.lte(chrono::Utc::now().timestamp()))
                .all(&get_db())
                .await
                .unwrap();
            for pod in pods {
                cluster::delete(pod.name.clone()).await;
                crate::model::pod::Entity::delete_by_id(pod.id)
                    .exec(&get_db())
                    .await
                    .unwrap();
                info!("Cleaned up expired cluster: {0}", pod.name);
            }
            tokio::time::sleep(interval).await;
        }
    });
}
