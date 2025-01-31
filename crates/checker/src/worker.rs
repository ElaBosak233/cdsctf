use std::time;

use crate::get_checker_context;

pub async fn cleaner() {
    tokio::spawn(async move {
        let interval = time::Duration::from_secs(15 * 60);
        loop {
            let now = chrono::Utc::now();
            get_checker_context().retain(|id, ctx| {
                let duration = now.signed_duration_since(ctx.created_at);
                duration.num_hours() > 1
            });
            tokio::time::sleep(interval).await;
        }
    });
}
