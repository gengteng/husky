use infer_total::InferQueryGroup;
use vm::{StackIdx, VMResult};

use crate::qual::QualTable;

use super::*;

pub(super) struct EagerStmtParser<'a> {
    pub(super) db: &'a dyn InferQueryGroup,
    pub(super) arena: &'a RawExprArena,
    pub(super) variables: Vec<EagerVariable>,
    pub(super) file: FilePtr,
    pub(super) qual_table: QualTable,
}

impl<'a> EagerStmtParser<'a> {
    pub(super) fn new(
        input_placeholders: &[InputPlaceholder],
        db: &'a dyn InferQueryGroup,
        arena: &'a RawExprArena,
        file: FilePtr,
    ) -> Self {
        Self {
            db,
            arena,
            variables: input_placeholders
                .iter()
                .map(|input_placeholder| EagerVariable::from_input(input_placeholder))
                .collect(),
            file,
            qual_table: Default::default(),
        }
    }

    pub(super) fn def_variable(
        &mut self,
        varname: CustomIdentifier,
        ty: ScopePtr,
        qual: Qual,
    ) -> VMResult<StackIdx> {
        let varidx = StackIdx::new(self.variables.len())?;
        self.variables.push(EagerVariable {
            ident: varname,
            ty,
            qual,
        });
        Ok(varidx)
    }

    pub(super) fn varidx(&self, varname: CustomIdentifier) -> StackIdx {
        StackIdx::new(
            self.variables.len()
                - 1
                - self
                    .variables
                    .iter()
                    .rev()
                    .position(|v| v.ident == varname)
                    .unwrap(),
        )
        .unwrap()
    }
}

impl<'a> EagerExprParser<'a> for EagerStmtParser<'a> {
    fn arena(&self) -> &'a RawExprArena {
        self.arena
    }

    fn db(&self) -> &'a dyn InferQueryGroup {
        self.db
    }

    fn file(&self) -> FilePtr {
        self.file
    }
}