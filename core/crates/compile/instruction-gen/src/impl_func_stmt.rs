use crate::*;
use avec::Avec;
use vm::{InitKind, Instruction, InstructionVariant, VMConditionBranch, VMPatternBranch};

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_func_stmts(&mut self, stmts: &[Arc<FuncStmt>]) {
        stmts
            .iter()
            .for_each(|stmt| self.compile_func_stmt(stmt.clone()));
    }

    fn compile_func_stmt(&mut self, stmt: Arc<FuncStmt>) {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.compile_eager_expr(initial_value, self.sheet.variable_stack.next_stack_idx());
                self.def_variable(varname.ident)
            }
            FuncStmtVariant::Assert { ref condition } => {
                self.compile_eager_expr(condition, self.sheet.variable_stack.next_stack_idx());
                self.push_instruction(Instruction::new(InstructionVariant::Assert, stmt))
            }
            FuncStmtVariant::Return { ref result } => {
                self.compile_eager_expr(result, self.sheet.variable_stack.next_stack_idx());
                self.push_instruction(Instruction::new(InstructionVariant::Return, stmt));
            }
            FuncStmtVariant::ConditionFlow { ref branches } => {
                self.push_instruction(Instruction::new(
                    InstructionVariant::ConditionFlow {
                        branches: self.compile_func_condition_flow(branches),
                    },
                    stmt,
                ))
            }
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => {
                self.compile_eager_expr(match_expr, self.sheet.variable_stack.next_stack_idx());
                self.push_instruction(Instruction::new(
                    InstructionVariant::PatternMatch {
                        branches: self.compile_func_pattern_match(branches),
                    },
                    stmt,
                ))
            }
            FuncStmtVariant::ReturnXml { ref xml_expr } => {
                self.compile_xml_expr(xml_expr.clone());
                self.push_instruction(Instruction::new(InstructionVariant::Return, stmt));
            }
        }
    }

    fn compile_func_condition_flow(
        &self,
        branches: &[Arc<FuncConditionBranch>],
    ) -> Avec<VMConditionBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| match branch.variant {
                    FuncConditionBranchVariant::If { ref condition } => {
                        Arc::new(VMConditionBranch {
                            opt_condition_sheet: {
                                let mut condition_sheet_builder = self.subsheet_builder();
                                condition_sheet_builder.compile_eager_expr(
                                    condition,
                                    condition_sheet_builder
                                        .sheet
                                        .variable_stack
                                        .next_stack_idx(),
                                );
                                Some(condition_sheet_builder.finalize())
                            },
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    FuncConditionBranchVariant::Elif { ref condition } => {
                        Arc::new(VMConditionBranch {
                            opt_condition_sheet: {
                                let mut condition_sheet_builder = self.subsheet_builder();
                                condition_sheet_builder.compile_eager_expr(
                                    condition,
                                    condition_sheet_builder
                                        .sheet
                                        .variable_stack
                                        .next_stack_idx(),
                                );
                                Some(condition_sheet_builder.finalize())
                            },
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        })
                    }
                    FuncConditionBranchVariant::Else => Arc::new(VMConditionBranch {
                        opt_condition_sheet: None,
                        body: {
                            let mut body_sheet = self.subsheet_builder();
                            body_sheet.compile_func_stmts(&branch.stmts);
                            body_sheet.finalize()
                        },
                    }),
                })
                .collect(),
        )
    }

    fn compile_func_pattern_match(
        &self,
        branches: &[Arc<FuncPatternBranch>],
    ) -> Avec<VMPatternBranch> {
        Arc::new(
            branches
                .iter()
                .map(|branch| {
                    Arc::new(match branch.variant {
                        FuncPatternBranchVariant::Case { ref pattern } => VMPatternBranch {
                            opt_pattern: Some(pattern.compile()),
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                        FuncPatternBranchVariant::Default => VMPatternBranch {
                            opt_pattern: None,
                            body: {
                                let mut body_sheet = self.subsheet_builder();
                                body_sheet.compile_func_stmts(&branch.stmts);
                                body_sheet.finalize()
                            },
                        },
                    })
                })
                .collect(),
        )
    }
}