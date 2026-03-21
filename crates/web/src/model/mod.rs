//! Shared JSON models referenced by multiple HTTP handlers / OpenAPI schemas.

use serde::{Deserialize, Serialize};

/// Lightweight file descriptor returned when listing or uploading media-like
/// resources.
#[derive(Clone, Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Metadata {
    /// Original filename as provided by the client or storage layer.
    pub filename: String,
    /// Byte length of the object.
    pub size: u64,
}
