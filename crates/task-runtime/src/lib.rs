//! Generic task runtime: SQLite persistence, FIFO scheduling with a
//! concurrency cap, status state machine and broadcast events.
//!
//! The runtime knows nothing about content: it hands opaque `TaskDraft`s to
//! providers and records the generic progress and artifacts they report.

mod error;
mod model;
mod runtime;
mod storage;

pub use error::RuntimeError;
pub use model::*;
pub use runtime::TaskRuntime;
