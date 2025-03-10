mod traits;
mod worker;

use lettre::{
    AsyncSmtpTransport, Tokio1Executor,
    transport::smtp::{
        authentication::Credentials,
        client::{Tls, TlsParameters},
    },
};

use crate::traits::EmailError;

pub fn get_mailer() -> Result<AsyncSmtpTransport<Tokio1Executor>, EmailError> {
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
