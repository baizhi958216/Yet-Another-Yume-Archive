//! Reusable HTTP transfer engine for providers: probe, segmented parallel
//! download with resume sidecars, streaming fallback.
//!
//! Invariant: this crate never depends on `yaya-provider-api`, and hosts never
//! depend on this crate — only providers do.

mod engine;
mod error;
mod model;
mod resume;

pub use engine::DownloadEngine;
pub use error::DownloadError;
pub use model::{DownloadOptions, DownloadProgress, DownloadResult, ResourceSpec};
