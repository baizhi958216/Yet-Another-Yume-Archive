//! Host-agnostic command layer. Every user-facing operation is an async fn
//! on [`AppCore`]; `src-tauri` commands and `src-web` REST handlers are thin
//! wrappers around it.

mod commands;
mod error;
mod settings;
mod state;

pub use error::AppError;
pub use settings::AppSettings;
pub use state::{AppCore, AppPaths};
