use thiserror::Error;

#[derive(Error, Debug)]
pub enum EmailError {
    #[error("lettre error: {0}")]
    LettreError(#[from] lettre::error::Error),
    #[error("smtp error: {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    #[error("mailer test error")]
    MailerTestError(),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}
