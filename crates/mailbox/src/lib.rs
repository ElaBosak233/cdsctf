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

/// JSON body published to the `mailbox` queue subject (see `cds-worker`
/// consumer).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Payload {
    pub name: String,
    pub email: String,
    pub subject: String,
    pub body: String,
}

/// SMTP-backed mail API (configuration from DB). Queue subscription lives in
/// `cds-worker`.
#[derive(Debug, Clone)]
pub struct Mailbox {
    db: DB,
}

impl Mailbox {
    pub fn new(db: DB) -> Self {
        Self { db }
    }

    /// Send one message described by a queue [`Payload`].
    pub async fn send_payload(&self, payload: &Payload) -> Result<(), MailboxError> {
        let to = LMailbox::new(
            Some(payload.name.clone()),
            payload.email.parse::<Address>()?,
        );
        self.send(to, &payload.subject, &payload.body).await
    }

    pub(crate) async fn inject(&self, body: &str) -> String {
        body.replace(
            "%TITLE%",
            &cds_db::get_config(&self.db.conn).await.meta.title,
        )
    }

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
