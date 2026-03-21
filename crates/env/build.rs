//! Cargo build script for this crate (code generation and build-time metadata).

/// Runs `shadow-rs` to embed version, commit, and build-time metadata for
/// `cds_env::get_*` helpers.
fn main() {
    shadow_rs::ShadowBuilder::builder().build().unwrap();
}
