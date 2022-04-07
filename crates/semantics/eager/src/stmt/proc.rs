mod branch;
mod loop_kind;
mod parse;

use std::sync::Arc;

pub use branch::*;
use fold::Indent;
pub use loop_kind::*;
use vm::{InitKind, InstructionId, InstructionSource, StackIdx};

use super::*;
use crate::*;

use parser::EagerStmtParser;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcStmt {
    pub kind: ProcStmtKind,
    pub file: FilePtr,
    pub range: TextRange,
    pub indent: Indent,
    pub instruction_id: InstructionId,
}

impl InstructionSource for ProcStmt {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProcStmtKind {
    Init {
        varname: CustomIdentifier,
        initial_value: Arc<EagerExpr>,
        init_kind: InitKind,
        varidx: StackIdx,
    },
    Assert {
        condition: Arc<EagerExpr>,
    },
    Execute {
        expr: Arc<EagerExpr>,
    },
    Return {
        result: Arc<EagerExpr>,
    },
    BranchGroup {
        kind: ImprBranchGroupKind,
        branches: Vec<Arc<ImprBranch>>,
    },
    Loop {
        loop_kind: LoopKind,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
}

pub fn parse_impr_stmts(
    input_placeholders: &[InputPlaceholder],
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    iter: fold::FoldIter<AstResult<Ast>, fold::FoldedList<AstResult<Ast>>>,
    file: FilePtr,
) -> SemanticResultArc<Vec<Arc<ProcStmt>>> {
    EagerStmtParser::new(input_placeholders, db, arena, file).parse_impr_stmts(iter)
}