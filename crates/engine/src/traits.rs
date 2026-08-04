//! Shared error and diagnostic types for the Lua engine.

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("lua error: {0}")]
    LuaError(#[from] mlua::Error),
    #[error("lua diagnostics error")]
    DiagnosticsError(Vec<DiagnosticMarker>),
    #[error("missing context: {0}")]
    MissingContext(String),
    #[error("missing function: {0}")]
    MissingFunction(String),
    #[error("missing script: {0}")]
    MissingScript(String),
    #[error("script error: {0}")]
    ScriptError(String),
    #[error("script execution timed out")]
    Timeout,
    #[error(transparent)]
    OtherError(#[from] anyhow::Error),
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub struct DiagnosticMarker {
    pub kind: DiagnosticKind,
    pub message: String,
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    Error,
    Warning,
}
