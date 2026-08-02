use std::sync::Arc;

use async_trait::async_trait;
use tokio_util::sync::CancellationToken;
use yaya_provider_api::{Artifact, ProgressReporter, ProviderError};

use crate::TaskSnapshot;

/// Optional host-owned final publishing step for platforms whose public file
/// storage cannot be represented by a normal filesystem path.
#[async_trait]
pub trait ArtifactPublisher: Send + Sync {
    async fn publish(
        &self,
        task: &TaskSnapshot,
        artifacts: Vec<Artifact>,
        reporter: Arc<dyn ProgressReporter>,
        cancellation: CancellationToken,
    ) -> Result<Vec<Artifact>, ProviderError>;
}
