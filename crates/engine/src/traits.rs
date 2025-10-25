use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("rune context error: {0}")]
    ContextError(#[from] rune::ContextError),
    #[error("rune source error: {0}")]
    SourceError(#[from] rune::source::FromPathError),
    #[error("rune build error: {0}")]
    BuildError(#[from] rune::BuildError),
    #[error("rune alloc error: {0}")]
    AllocError(#[from] rune::alloc::Error),
    #[error("rune vm error: {0}")]
    RuntimeError(#[from] rune::runtime::RuntimeError),
    #[error("rune vm error: {0}")]
    VmError(#[from] rune::runtime::VmError),
    #[error("rune diagnostics error")]
    DiagnosticsError(Vec<DiagnosticMarker>),
    #[error("compile error: {0}")]
    CompileError(String),
    #[error("missing context: {0}")]
    MissingContext(String),
    #[error("missing function: {0}")]
    MissingFunction(String),
    #[error("missing script: {0}")]
    MissingScript(String),
    #[error("script error: {0}")]
    ScriptError(String),
    #[error("String UTF-8 decode error: {0}")]
    FromUtf8Error(#[from] std::string::FromUtf8Error),
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct DiagnosticMarker {
    pub kind: DiagnosticKind,
    pub message: String,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    Error,
    Warning,
}
