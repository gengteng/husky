use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FeatureEvalId(pub(crate) usize);

const NEXT_RAW_ID: AtomicUsize = AtomicUsize::new(0);

impl Default for FeatureEvalId {
    fn default() -> Self {
        Self(NEXT_RAW_ID.fetch_add(1, Ordering::SeqCst))
    }
}