use husky_vm::FrameKind;

use super::*;

impl<'a> TraceLineBuilder<'a> {
    pub(crate) fn proc_stmt_tokens(&mut self, stmt: &ProcStmt, history: &Arc<History<'static>>) {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => {
                self.gen_keyword_token(
                    match init_kind {
                        InitKind::Let => "let ",
                        InitKind::Var => "var ",
                        InitKind::Decl => panic!(),
                    },
                    None,
                    None,
                );
                self.gen_ident_token(varname.ident.0, None, None);
                self.gen_special_token(" = ", None, None);
                self.gen_eager_expr_tokens(initial_value, history, ExprTokenConfig::stmt())
            }
            ProcStmtVariant::Assert { ref condition } => {
                self.gen_keyword_token("assert ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::stmt())
            }
            ProcStmtVariant::Execute { ref expr } => {
                self.gen_eager_expr_tokens(expr, history, ExprTokenConfig::exec());
                match expr.variant {
                    EagerExprVariant::Opn {
                        ref opn_variant, ..
                    } => match opn_variant {
                        EagerOpnVariant::Binary { opr } => match opr {
                            BinaryOpr::Assign(_) => {
                                if let Some(_) = history.register_result(expr) {
                                    self.gen_fade_assign_token()
                                }
                            }
                            BinaryOpr::Pure(_) => (),
                        },
                        _ => (),
                    },
                    _ => panic!(),
                }
            }
            ProcStmtVariant::Return { ref result, .. } => {
                self.gen_keyword_token("return ", None, None);
                self.gen_eager_expr_tokens(result, history, ExprTokenConfig::stmt())
            }
            ProcStmtVariant::ConditionFlow { .. } => todo!(),
            ProcStmtVariant::Loop {
                ref loop_variant, ..
            } => self.loop_stmt_tokens(stmt, loop_variant, history),
            ProcStmtVariant::Break => self.gen_keyword_token("break", None, None),
            ProcStmtVariant::Match { .. } => todo!(),
        }
    }

    fn loop_stmt_tokens(
        &mut self,
        stmt: &ProcStmt,
        loop_variant: &LoopVariant,
        history: &Arc<History<'static>>,
    ) {
        match loop_variant {
            LoopVariant::For {
                frame_var,
                ref initial_boundary,
                ref final_boundary,
                ..
            } => {
                self.gen_keyword_token("for ", None, None);
                self.initial_boundary_tokens(initial_boundary, history);
                self.gen_ident_token(frame_var.ident.0, None, Some(frame_var.range.start));
                self.final_boundary_tokens(final_boundary, history);
                self.gen_special_token(":", None, None);
            }
            LoopVariant::ForExt {
                frame_var,
                ref final_boundary,
                ..
            } => {
                self.gen_keyword_token("forext ", None, None);
                self.gen_ident_token(frame_var.ident.0, None, Some(frame_var.range.start));
                self.final_boundary_tokens(final_boundary, history);
                self.gen_special_token(":", None, None);
            }
            LoopVariant::While { ref condition } => {
                self.gen_keyword_token("while ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::loop_head());
                self.gen_special_token(":", None, None);
            }
            LoopVariant::DoWhile { condition } => {
                self.gen_keyword_token("do while ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::loop_head());
                self.gen_special_token(":", None, None);
            }
        }
        if let Some(entry) = history.get(stmt) {
            match entry {
                HistoryEntry::Loop { control, .. } => self.add_control_tokens(control),
                _ => panic!(),
            }
        }
    }

    fn initial_boundary_tokens(&mut self, boundary: &Boundary, history: &Arc<History<'static>>) {
        match boundary.opt_bound {
            Some(ref bound) => {
                self.gen_eager_expr_tokens(bound, history, ExprTokenConfig::stmt());
                match boundary.kind {
                    BoundaryKind::UpperOpen => self.gen_special_token(" > ", None, None),
                    BoundaryKind::UpperClosed => self.gen_special_token(" >= ", None, None),
                    BoundaryKind::LowerOpen => self.gen_special_token(" < ", None, None),
                    BoundaryKind::LowerClosed => self.gen_special_token(" <= ", None, None),
                }
            }
            None => (),
        }
    }

    fn final_boundary_tokens(&mut self, boundary: &Boundary, history: &Arc<History<'static>>) {
        match boundary.opt_bound {
            Some(ref bound) => {
                self.gen_special_token(
                    match boundary.kind {
                        BoundaryKind::UpperOpen => " < ",
                        BoundaryKind::UpperClosed => " <= ",
                        BoundaryKind::LowerOpen => " > ",
                        BoundaryKind::LowerClosed => " >= ",
                    },
                    None,
                    None,
                );
                self.gen_eager_expr_tokens(bound, history, ExprTokenConfig::stmt())
            }
            None => (),
        }
    }

    pub(crate) fn gen_loop_frame_tokens(&mut self, loop_frame_data: &LoopFrameData) {
        match loop_frame_data.frame_kind {
            FrameKind::For(frame_var) => {
                self.gen_keyword_token("frame ", None, None);
                self.gen_ident_token(&frame_var, None, None);
                self.gen_special_token(" = ", None, None);
                self.gen_literal_token(loop_frame_data.frame_var_value, None);
            }
            FrameKind::Loop => {
                self.gen_keyword_token("frame ", None, None);
                self.gen_literal_token(loop_frame_data.frame_var_value, None);
            }
        };
        self.add_control_tokens(&loop_frame_data.control)
    }

    pub(crate) fn gen_proc_branch_tokens(
        &mut self,
        stmt: &ProcStmt,
        branch: &ProcConditionFlowBranch,
        history: &Arc<History<'static>>,
    ) {
        match branch.variant {
            ProcConditionFlowBranchVariant::If { ref condition } => {
                self.gen_keyword_token("if ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.gen_special_token(":", None, None)
            }
            ProcConditionFlowBranchVariant::Elif { ref condition } => {
                self.gen_keyword_token("elif ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.gen_special_token(":", None, None)
            }
            ProcConditionFlowBranchVariant::Else => {
                self.gen_keyword_token("else", None, None);
                self.gen_special_token(":", None, None)
            }
        }
        if let Some(entry) = history.get(stmt) {
            match entry {
                HistoryEntry::ControlFlow { control, .. } => self.add_control_tokens(control),
                _ => todo!(),
            }
        }
    }
}
