//! Serde models exchanged between hosts and providers (camelCase on the wire).

mod artifact;
mod form;
mod progress;
mod task;
mod ui;
mod view;

pub use artifact::{Artifact, BinaryAsset};
pub use form::{FormControl, FormField, SelectOption};
pub use progress::TaskProgress;
pub use task::{ProviderTaskRequest, TaskDraft};
pub use ui::{
    ProviderUiActionRequest, ProviderUiBundle, ProviderUiDescriptor, ProviderUiSurface,
    PROVIDER_UI_API_VERSION,
};
pub use view::{ProviderInput, ProviderView};
