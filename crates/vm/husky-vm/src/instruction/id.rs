use std::{
    panic::RefUnwindSafe,
    sync::atomic::{AtomicUsize, Ordering},
};

use husky_file::FilePtr;
use husky_text::{FileRange, FileRanged, TextRange};

static NEXT_VM_INSTRUCTION_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InstructionId(pub(crate) usize);

pub trait InstructionSource: std::fmt::Debug + Send + Sync + RefUnwindSafe + FileRanged {
    fn instruction_id(&self) -> InstructionId;
}

impl Default for InstructionId {
    fn default() -> Self {
        let raw = NEXT_VM_INSTRUCTION_ID.fetch_add(1, Ordering::Relaxed);
        Self(raw)
    }
}
