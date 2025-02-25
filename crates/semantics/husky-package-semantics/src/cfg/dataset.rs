use std::sync::Arc;

use husky_eager_semantics::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConfig {
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
    // pub instruction_sheet: Arc<InstructionSheet>,
}

impl DatasetConfig {
    pub fn new(stmts: Arc<Vec<Arc<FuncStmt>>>) -> Self {
        Self { stmts }
    }
}

// instruction_sheet: InstructionSheetBuilder::new_decl(vec![], &stmts, false),
