use crate::*;
use husky_file::FilePtr;
use husky_text::TextRange;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcConditionFlowBranch {
    pub variant: ProcConditionFlowBranchVariant,
    pub stmts: Arc<Vec<Arc<ProcStmt>>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub idx: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcConditionFlowBranchVariant {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
}
