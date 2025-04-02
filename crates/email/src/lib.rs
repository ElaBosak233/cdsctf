mod traits;
mod util;

use anyhow::anyhow;
use lettre::{
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::header::ContentType,
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};

use crate::traits::EmailError;

pub async fn get_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, EmailError> {
    if !cds_config::get_variable().email.is_enabled {
        return Err(EmailError::OtherError(anyhow!("disabled")));
    }

    let credentials = Credentials::new(
        cds_config::get_variable().email.username.clone(),
        cds_config::get_variable().email.password.clone(),
    );

    let builder = match cds_config::get_variable().email.tls {
        cds_config::variable::email::Tls::Starttls => {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
                &cds_config::get_variable().email.host,
            )
        }
        cds_config::variable::email::Tls::Tls => Ok(AsyncSmtpTransport::<Tokio1Executor>::relay(
            &cds_config::get_variable().email.host,
        )?
        .tls(Tls::Wrapper(
            TlsParameters::builder(cds_config::get_variable().email.host.clone())
                .build()
                .unwrap(),
        ))),
        cds_config::variable::email::Tls::None => {
            Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                &cds_config::get_variable().email.host,
            ))
        }
    }?;

    let mailer: AsyncSmtpTransport<Tokio1Executor> = builder
        .port(cds_config::get_variable().email.port)
        .credentials(credentials)
        .timeout(Some(std::time::Duration::from_secs(10)))
        .build();

    Ok(mailer)
}

pub async fn send(to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
    let envelope = lettre::Message::builder()
        .from(
            format!(
                "{} <{}>",
                cds_config::get_variable().meta.title,
                cds_config::get_variable().email.username
            )
            .parse()
            .unwrap(),
        )
        .to(to.to_string().parse().unwrap())
        .header(ContentType::TEXT_HTML)
        .subject(util::inject(subject))
        .body(util::inject(body))?;

    get_mailer().await?.send(envelope).await?;

    Ok(())
}
