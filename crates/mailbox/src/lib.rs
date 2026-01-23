mod traits;
mod worker;

use anyhow::anyhow;
use cds_db::DB;
use cds_queue::Queue;
use lettre::{
    Address, AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::{Mailbox as LMailbox, SinglePart, header::ContentType},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};
use once_cell::sync::OnceCell;
pub use worker::Payload;

use crate::traits::MailboxError;

static MAILBOX: OnceCell<Mailbox> = OnceCell::new();

#[derive(Debug, Clone)]
pub struct Mailbox {
    db: DB,
    queue: Queue,
}

pub async fn init(db: &DB, queue: &Queue) -> Result<(), MailboxError> {
    let m = Mailbox {
        db: db.clone(),
        queue: queue.clone(),
    };

    worker::init(m.clone()).await;
    MAILBOX.set(m).expect("Mailbox already initialized");

    Ok(())
}

impl Mailbox {
    pub(crate) async fn inject(&self, body: &str) -> String {
        body.replace(
            "%TITLE%",
            &cds_db::get_config(&self.db.conn).await.meta.title,
        )
    }

    pub(crate) async fn get_mailer(
        &self,
    ) -> Result<AsyncSmtpTransport<Tokio1Executor>, MailboxError> {
        if !cds_db::get_config(&self.db.conn).await.email.is_enabled {
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
