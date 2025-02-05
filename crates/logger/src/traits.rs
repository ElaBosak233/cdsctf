use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LoggerError {
    #[error("appender error: {0}")]
    AppenderError(#[from] tracing_appender::rolling::InitError),
    #[error("io error: {0}")]
    IOError(#[from] io::Error),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}
