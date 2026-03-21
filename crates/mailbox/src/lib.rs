//! Outbound email over SMTP using settings stored in the database.
//!
//! # Architecture
//!
//! - The HTTP layer (or other code) **publishes** a JSON [`Payload`] to the
//!   NATS JetStream subject `mailbox` (consumed by the `cds-worker` crate’s
//!   `mailbox` module).
//! - This crate’s [`Mailbox`] type reads SMTP options from [`cds_db`] config
//!   and sends mail with [`lettre`].
//!
//! Template placeholders `%TITLE%` in subject/body are replaced with the
//! platform title from config (see [`Mailbox::inject`]).

/// Defines the `traits` submodule (see sibling `*.rs` files).
mod traits;

use anyhow::anyhow;
use cds_db::DB;
use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::{Mailbox as LMailbox, SinglePart, header::ContentType},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};
use serde::{Deserialize, Serialize};

use crate::traits::MailboxError;

/// JSON envelope for one outbound message, serialized on the `mailbox` queue
/// subject.
///
/// The worker deserializes this struct and calls [`Mailbox::send_payload`].
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    /// Display name shown to the recipient (e.g. user’s nickname).
    pub name: String,
    /// RFC 5322 email address of the recipient.
    pub email: String,
    /// Email subject line; may contain `%TITLE%` for substitution.
    pub subject: String,
    /// HTML body; may contain `%TITLE%` for substitution.
    pub body: String,
}

/// SMTP mailer bound to a [`DB`] handle (reads live config per send).
///
/// Clone is cheap: this is a thin handle around [`DB`].
#[derive(Debug, Clone)]
pub struct Mailbox {
    db: DB,
}

impl Mailbox {
    /// Creates a mailbox that will read SMTP and meta settings through `db`.
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    /// Sends one message described by a queue [`Payload`] (recipient +
    /// content).
    pub async fn send_payload(&self, payload: &Payload) -> Result<(), MailboxError> {
        // lettre’s Mailbox is the logical (name, address) pair for To/From headers.
        let to = LMailbox::new(
            Some(payload.name.clone()),
            payload.email.parse::<Address>()?,
        );
        self.send(to, &payload.subject, &payload.body).await
    }

    /// Replaces the `%TITLE%` placeholder with the configured site title from
    /// the database.
    pub(crate) async fn inject(&self, body: &str) -> String {
        body.replace(
            "%TITLE%",
            &cds_db::get_config(&self.db.conn).await.meta.title,
        )
    }

    /// Builds an async SMTP transport from DB `email` config.
    ///
    /// Returns an error if email is disabled in config or TLS/build steps fail.
    pub(crate) async fn get_mailer(
        &self,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, MailboxError> {
        if !cds_db::get_config(&self.db.conn).await.email.enabled {
            return Err(MailboxError::OtherError(anyhow!("disabled")));
        }

        let credentials = Credentials::new(
            cds_db::get_config(&self.db.conn)
                .await
                .email
                .username
                .clone(),
            cds_db::get_config(&self.db.conn)
                .await
                .email
                .password
                .clone(),
        );

        // Three TLS modes: upgrade (STARTTLS), implicit TLS wrapper, or plaintext
        // (dangerous).
        let builder = match cds_db::get_config(&self.db.conn).await.email.tls {
            cds_db::config::email::Tls::Starttls => {
                AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
                    &cds_db::get_config(&self.db.conn).await.email.host,
                )
            }
            cds_db::config::email::Tls::Tls => Ok(AsyncSmtpTransport::<Tokio1Executor>::relay(
                &cds_db::get_config(&self.db.conn).await.email.host,
            )?
            .tls(Tls::Wrapper(
                TlsParameters::builder(cds_db::get_config(&self.db.conn).await.email.host.clone())
                    .build()?,
            ))),
            cds_db::config::email::Tls::None => {
                Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                    &cds_db::get_config(&self.db.conn).await.email.host,
                ))
            }
        }?;

        let mailer: AsyncSmtpTransport<Tokio1Executor> = builder
            .port(cds_db::get_config(&self.db.conn).await.email.port)
            .credentials(credentials)
            .timeout(Some(std::time::Duration::from_secs(10)))
            .build();

        Ok(mailer)
    }

    /// Low-level send: builds a single-part HTML message and delivers it
    /// through [`get_mailer`].
    pub(crate) async fn send(
        &self,
        to: LMailbox,
        subject: &str,
        body: &str,
    ) -> Result<(), MailboxError> {
        let envelope = lettre::Message::builder()
            .from(LMailbox::new(
                Some(cds_db::get_config(&self.db.conn).await.meta.title),
                cds_db::get_config(&self.db.conn)
                    .await
                    .email
                    .username
                    .parse::<Address>()?,
            ))
            .to(to)
            .subject(self.inject(subject).await)
            .singlepart(
                SinglePart::builder()
                    .header(ContentType::TEXT_HTML)
                    .body(self.inject(body).await),
            )?;

        self.get_mailer().await?.send(envelope).await?;

        Ok(())
    }
}
