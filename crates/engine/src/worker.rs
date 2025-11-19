use time::OffsetDateTime;

use crate::get_global_engine;

pub async fn cleaner() {
    tokio::spawn(async move {
        let interval = std::time::Duration::from_mins(15); // 15 minutes
        loop {
            let now = OffsetDateTime::now_utc();
            get_global_engine().retain(|_id, ctx| {
                let duration = now - ctx.created_at;
                duration.whole_hours() > 1
            });
            tokio::time::sleep(interval).await;
        }
    });
}
