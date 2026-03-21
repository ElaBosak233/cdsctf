//! JetStream-backed **consumers**: one module per queue subject (`calculator`,
//! `checker`, `mailbox`).

use cds_checker::Checker;
use cds_db::DB;
use cds_mailbox::Mailbox;
use cds_queue::Queue;

pub mod calculator;
pub mod checker;
pub mod mailbox;

/// Start every queue consumer (calculator, checker, mailbox).
pub async fn init(
    db: &DB,
    queue: &Queue,
    checker: &Checker,
    mailbox: &Mailbox,
) -> Result<(), anyhow::Error> {
    crate::calculator::spawn(db, queue).await;
    crate::checker::spawn(db, queue, checker).await;
    crate::mailbox::spawn(queue, mailbox).await;
    Ok(())
}
