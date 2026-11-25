//! Background workers:
//!
//! | Module      | Mechanism                                      | Purpose                    |
//! |-------------|------------------------------------------------|----------------------------|
//! | [`checker`] | PG `FOR UPDATE SKIP LOCKED` + NATS Core wakeup | Asynchronous flag judging  |
//! | [`mailbox`] | JetStream `cds.mail.send`                      | Outbound SMTP delivery     |
//!
//! Dynamic scoring is no longer a background mutating worker — it is served
//! directly as a CQRS read model by [`cds_score::Score`].

use cds_checker::Checker;
use cds_db::DB;
use cds_mailbox::Mailbox;
use cds_queue::Queue;
use cds_score::Score;

/// Pull-on-capacity submission checker worker backed by PostgreSQL.
pub mod checker;

/// JetStream consumer for outbound SMTP mail (`cds.mail.send`).
pub mod mailbox;

/// Starts all background worker tasks (`checker`, `mailbox`).
#[tracing::instrument(skip_all, fields(handler = "init"))]
pub async fn init(
    db: &DB,
    queue: &Queue,
    checker: &Checker,
    score: &Score,
    mailbox: &Mailbox,
) -> Result<(), anyhow::Error> {
    crate::checker::spawn(db, queue, checker, score).await;
    crate::mailbox::spawn(queue, mailbox).await;
    Ok(())
}
