use thiserror::Error;

#[derive(Debug, Error)]
pub enum CheckerError {
    #[error("rune context error: {0}")]
    ContextError(#[from] rune::ContextError),
    #[error("rune source error: {0}")]
    SourceError(#[from] rune::source::FromPathError),
    #[error("rune build error: {0}")]
    BuildError(#[from] rune::BuildError),
    #[error("rune alloc error: {0}")]
    AllocError(#[from] rune::alloc::Error),
    #[error("rune runtime error: {0}")]
    RuntimeError(#[from] rune::runtime::VmError),
    #[error("rune diagnostics error: {0}")]
    DiagnosticsError(#[from] rune::diagnostics::EmitError),
    #[error("compile error: {0}")]
    CompileError(String),
    #[error("missing function: {0}")]
    MissingFunction(String),
    #[error("missing script: {0}")]
    MissingScript(String),
    #[error("script error: {0}")]
    ScriptError(String),
    #[error("String UTF-8 decode error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    MediaError(#[from] cds_media::traits::MediaError),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}
