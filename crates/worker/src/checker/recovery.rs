//! Recover the database-to-message publication gap using queued submissions.

use std::{sync::Arc, time::Duration};

use time::OffsetDateTime;
use tracing::{Instrument, debug, error, info};

use super::{Context, SUBJECT};

const SCAN_INTERVAL: Duration = Duration::from_secs(10);
const PUBLICATION_GRACE: time::Duration = time::Duration::seconds(10);
const BATCH_SIZE: u64 = 128;

#[cfg(test)]
mod tests;

/// A fixed cutoff and keyset pagination let later pages progress even when
/// earlier queued rows are still waiting for checker capacity. Duplicate
/// messages use the normal atomic claim and conditional finalization path.
pub(super) async fn republish_queued(
    ctx: &Context,
    cutoff: OffsetDateTime,
) -> Result<usize, anyhow::Error> {
    let mut after_id = None;
    let mut published = 0;
    loop {
        let ids =
            cds_db::submission::find_queued_ids_before(&ctx.db.conn, cutoff, after_id, BATCH_SIZE)
                .await?;
        let Some(&last_id) = ids.last() else {
            return Ok(published);
        };
        let count = ids.len();
        for id in ids {
            ctx.queue.publish(SUBJECT, id, None).await?;
            published += 1;
        }
        after_id = Some(last_id);
        if count < BATCH_SIZE as usize {
            return Ok(published);
        }
    }
}

pub(super) async fn run(ctx: Arc<Context>) {
    // Startup retains lease recovery. Periodic scans only publish Queued work;
    // they do not reset another worker's processing ownership.
    match cds_db::submission::reset_stale_processing(&ctx.db.conn).await {
        Ok(reset) => debug!(reset, "startup processing leases recovered"),
        Err(error) => error!(?error, "startup processing recovery failed"),
    }

    let mut interval = tokio::time::interval(SCAN_INTERVAL);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut startup = true;
    loop {
        interval.tick().await;
        let cutoff = if startup {
            startup = false;
            OffsetDateTime::now_utc()
        } else {
            OffsetDateTime::now_utc() - PUBLICATION_GRACE
        };
        match republish_queued(&ctx, cutoff)
            .instrument(
                tracing::info_span!(parent: None, "submission.recover", operator_kind = "system"),
            )
            .await
        {
            Ok(0) => debug!("queued publication scan completed"),
            Ok(count) => info!(count, subject = SUBJECT, "queued submissions republished"),
            Err(error) => error!(
                ?error,
                "queued publication recovery failed; retrying next scan"
            ),
        }
    }
}
