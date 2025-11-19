use thiserror::Error;

#[derive(Debug, Error)]
pub enum ObserveError {
    #[error("no instance")]
    NoInstance,

    #[error("exporter build error: {0}")]
    ExporterBuildError(#[from] opentelemetry_otlp::ExporterBuildError),
    #[error("tokio join error")]
    TokioJoinError(#[from] tokio::task::JoinError),
    #[error("other error: {0}")]
    OtherError(#[from] anyhow::Error),
}
