use crate::model::TaskProgress;

pub trait ProgressReporter: Send + Sync {
    fn report(&self, progress: TaskProgress);
}

pub struct NoopReporter;

impl ProgressReporter for NoopReporter {
    fn report(&self, _progress: TaskProgress) {}
}
