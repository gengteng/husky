use std::sync::Arc;

use vm::{Instruction, InstructionKind, InstructionSheet};

use semantics_eager::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DatasetConfig {
    pub stmts: Arc<Vec<Arc<DeclStmt>>>,
    pub instruction_sheet: Arc<InstructionSheet>,
}

impl DatasetConfig {
    pub fn new(stmts: Arc<Vec<Arc<DeclStmt>>>) -> Self {
        Self {
            instruction_sheet: InstructionSheetBuilder::new_decl(vec![], &stmts, false),
            stmts,
        }
    }
}