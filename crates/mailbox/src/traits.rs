//! Error types for SMTP and address handling in [`crate::Mailbox`].

use thiserror::Error;

/// Failure modes when building or using the SMTP client.
#[derive(Error, Debug)]
pub enum MailboxError {
    /// Underlying `lettre` message/build error.
    #[error("lettre error: {0}")]
    LettreError(#[from] lettre::error::Error),
    /// Recipient or sender address could not be parsed.
    #[error("address error: {0}")]
    AddressError(#[from] lettre::address::AddressError),
    /// SMTP session or relay error after connection.
    #[error("smtp error: {0}")]
    SmtpError(#[from] lettre::transport::smtp::Error),
    /// Reserved for connection tests that fail without a more specific variant.
    #[error("mailer test error")]
    MailerTestError(),
    /// Catch-all for disabled mail, timeouts wrapped as `anyhow`, etc.
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}
