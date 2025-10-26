use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("lettre error: {0}")]
    LettreError(#[from] lettre::error::Error),
    #[error("address error: {0}")]
    AddressError(#[from] lettre::address::AddressError),
    #[error("smtp error: {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    #[error("mailer test error")]
    MailerTestError(),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}
