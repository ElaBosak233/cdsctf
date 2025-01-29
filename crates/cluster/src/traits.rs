use thiserror::Error;

#[derive(Error, Debug)]
pub enum ClusterError {
    #[error("kube error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("failed to infer config: {0}")]
    InferConfigError(#[from] kube::config::InferConfigError),
    #[error("failed to load kube config: {0}")]
    KubeConfigError(#[from] kube::config::KubeconfigError),
    #[error("kube runtime wait error: {0}")]
    KubeRuntimeWaitError(#[from] kube::runtime::wait::Error),
    #[error("proxy error: {0}")]
    ProxyError(#[from] wsrx::Error),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("missing field: {0}")]
    MissingField(String),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}
