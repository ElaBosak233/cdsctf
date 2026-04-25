//! Background JetStream **consumers**: one Tokio task per logical queue
//! subject.
//!
//! | Module       | Subject      | Purpose                                      |
//! |-------------|--------------|----------------------------------------------|
//! | [`calculator`] | `calculator` | Recompute dynamic scores/ranks after solves |
//! | [`checker`]  | `checker`    | Run asynchronous flag checks                |
//! | [`mailbox`]  | `mailbox`    | Deliver outbound SMTP mail                  |

use cds_checker::Checker;
use cds_db::DB;
use cds_mailbox::Mailbox;
use cds_queue::Queue;

/// Defines the `calculator` submodule (see sibling `*.rs` files).
pub mod calculator;

/// Defines the `checker` submodule (see sibling `*.rs` files).
pub mod checker;

/// Defines the `mailbox` submodule (see sibling `*.rs` files).
pub mod mailbox;

/// Start every queue consumer (calculator, checker, mailbox).
#[tracing::instrument(skip_all, fields(handler = "init"))]
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
