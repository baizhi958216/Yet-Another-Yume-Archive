//! Serde models exchanged between hosts and providers (camelCase on the wire).

mod artifact;
mod auth;
mod form;
mod progress;
mod settings;
mod task;
mod view;

pub use artifact::{Artifact, BinaryAsset};
pub use auth::{ProviderAuthActionRequest, ProviderAuthPage};
pub use form::{FormControl, FormField, SelectOption};
pub use progress::TaskProgress;
pub use settings::{
    ProviderSettingAction, ProviderSettingActionStyle, ProviderSettingStatus,
    ProviderSettingsActionRequest, ProviderSettingsActionResult, ProviderSettingsPage,
    ProviderSettingsSection, ProviderSettingsState, ProviderSettingsView,
};
pub use task::{ProviderTaskRequest, TaskDraft};
pub use view::{ProviderInput, ProviderView};
