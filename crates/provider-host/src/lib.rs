//! Loads providers into a host: manifest discovery, spawn-per-call subprocess
//! transport (desktop/web) and in-process hosting (mobile/builtin).
//!
//! The wire protocol is specified in `docs/provider-protocol.md`.

mod discovery;
mod error;
mod external;
mod hosted;
mod manager;
mod manifest;
mod protocol;

pub use discovery::{scan, scan_roots, target_key};
pub use error::HostError;
pub use external::ExternalProvider;
pub use hosted::{HostedProvider, ProviderControl, ProviderDescriptor};
pub use manager::ProviderManager;
pub use manifest::{MatchRule, ProviderInfo, ProviderManifest, ProviderUiManifest};
pub use protocol::PROVIDER_PROTOCOL_VERSION;
