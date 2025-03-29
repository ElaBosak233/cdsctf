mod traits;
mod util;

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Tokio1Executor,
    message::header::ContentType,
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};
use once_cell::sync::OnceCell;
use tracing::info;

use crate::traits::EmailError;

static MAILER: OnceCell<AsyncSmtpTransport<Tokio1Executor>> = OnceCell::new();

pub fn get_mailer() -> &'static AsyncSmtpTransport<Tokio1Executor> {
    &MAILER.get().unwrap()
}

pub async fn init() -> Result<(), EmailError> {
    if !cds_config::get_constant().email.is_enabled {
        return Ok(());
    }

    let credentials = Credentials::new(
        cds_config::get_constant().email.username.clone(),
        cds_config::get_constant().email.password.clone(),
    );

    let builder = match cds_config::get_constant().email.tls {
        cds_config::constant::email::Tls::Starttls => {
            AsyncSmtpTransport::<Tokio1Executor>::starttls_relay(
                &cds_config::get_constant().email.host,
            )
        }
        cds_config::constant::email::Tls::Tls => Ok(AsyncSmtpTransport::<Tokio1Executor>::relay(
            &cds_config::get_constant().email.host,
        )?
        .tls(Tls::Wrapper(
            TlsParameters::builder(cds_config::get_constant().email.host.clone())
                .build()
                .unwrap(),
        ))),
        cds_config::constant::email::Tls::None => {
            Ok(AsyncSmtpTransport::<Tokio1Executor>::builder_dangerous(
                &cds_config::get_constant().email.host,
            ))
        }
    }?;

    let mailer: AsyncSmtpTransport<Tokio1Executor> = builder
        .port(cds_config::get_constant().email.port)
        .credentials(credentials)
        .timeout(Some(std::time::Duration::from_secs(10)))
        .build();

    info!("Testing mailer, please wait for a few seconds...");

    mailer
        .test_connection()
        .await
        .map_err(|_| EmailError::MailerTestError())?;

    MAILER.set(mailer).ok();

    info!("Email mailer initialized successfully.");

    Ok(())
}

pub async fn send(to: &str, subject: &str, body: &str) -> Result<(), EmailError> {
    let envelope = lettre::Message::builder()
        .from(
            format!(
                "{} <{}>",
                cds_config::get_variable().meta.title,
                cds_config::get_constant().email.username
            )
            .parse()
            .unwrap(),
        )
        .to(format!("{}", to).parse().unwrap())
        .header(ContentType::TEXT_HTML)
        .subject(util::inject(subject))
        .body(util::inject(body))?;

    get_mailer().send(envelope).await?;

    Ok(())
}
