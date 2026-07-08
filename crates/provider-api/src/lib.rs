//! Content-agnostic vocabulary shared between YAYA hosts and providers.
//!
//! Hosts depend on this crate to talk to providers; providers implement
//! [`Provider`] against it. Nothing in here may know about any concrete site.

pub mod error;
pub mod model;
mod registry;
mod reporter;

use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;

pub use error::{ProviderError, ProviderErrorCode};
pub use model::*;
pub use registry::ProviderRegistry;
pub use reporter::{NoopReporter, ProgressReporter};

#[async_trait]
pub trait Provider: Send + Sync {
    fn id(&self) -> &str;

    /// Whether this provider can handle the given raw user input.
    fn supports(&self, input: &str) -> bool;

    /// Higher wins when several providers support the same input.
    fn priority(&self) -> i32 {
        0
    }

    /// Turn raw user input into a host-renderable view of downloadable tasks.
    async fn inspect(&self, input: ProviderInput) -> Result<ProviderView, ProviderError>;

    /// Execute one task, reporting progress and honouring cancellation.
    async fn run(
        &self,
        request: ProviderTaskRequest,
        reporter: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<Vec<Artifact>, ProviderError>;

    /// Fetch a small binary asset (e.g. a cover image) on the provider's
    /// network context. Optional capability.
    async fn fetch_asset(&self, _url: &str) -> Result<BinaryAsset, ProviderError> {
        Err(ProviderError::unsupported_capability(self.id(), "assets"))
    }
}
