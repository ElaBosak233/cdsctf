use anyhow::anyhow;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::{SinglePart, header::ContentType},
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};

use crate::{traits::EmailError, util};

pub(crate) async fn inject(body: &str) -> String {
    body.replace("%TITLE%", &cds_db::get_config().await.meta.title)
}

pub(crate) async fn get_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, EmailError> {
    if !cds_db::get_config().await.email.is_enabled {
        return Err(EmailError::OtherError(anyhow!("disabled")));
    }

    let credentials = Credentials::new(
        cds_db::get_config().await.email.username.clone(),
        cds_db::get_config().await.email.password.clone(),
    );

    let builder = match cds_db::get_config().await.email.tls {
        cds_db::entity::config::email::Tls::Starttls => {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
                &cds_db::get_config().await.email.host,
            )
        }
        cds_db::entity::config::email::Tls::Tls => Ok(AsyncSmtpTransport::<Tokio1Executor>::relay(
            &cds_db::get_config().await.email.host,
        )?
        .tls(Tls::Wrapper(
            TlsParameters::builder(cds_db::get_config().await.email.host.clone())
                .build()
                .unwrap(),
        ))),
        cds_db::entity::config::email::Tls::None => {
            Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                &cds_db::get_config().await.email.host,
            ))
        }
    }?;

    let mailer: AsyncSmtpTransport<Tokio1Executor> = builder
        .port(cds_db::get_config().await.email.port)
        .credentials(credentials)
        .timeout(Some(std::time::Duration::from_secs(10)))
        .build();

    Ok(mailer)
}

pub(crate) async fn send(to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
    let envelope = lettre::Message::builder()
        .from(
            format!(
                "{} <{}>",
                cds_db::get_config().await.meta.title,
                cds_db::get_config().await.email.username
            )
            .parse()
            .unwrap(),
        )
        .to(to.to_string().parse().unwrap())
        .subject(util::inject(subject).await)
        .singlepart(
            SinglePart::builder()
                .header(ContentType::TEXT_HTML)
                .body(util::inject(body).await),
        )?;

    get_mailer().await?.send(envelope).await?;

    Ok(())
}
