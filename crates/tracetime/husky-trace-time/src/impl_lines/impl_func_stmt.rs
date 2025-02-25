use super::*;

impl<'a> TraceLineBuilder<'a> {
    pub(crate) fn func_stmt_tokens(&mut self, stmt: &FuncStmt, history: &Arc<History<'static>>) {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                self.gen_ident_token(varname.ident.0, None, None);
                self.gen_special_token(" = ", None, None);
                self.gen_eager_expr_tokens(initial_value, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Assert { ref condition } => {
                self.gen_keyword_token("assert ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Require { ref condition, .. } => {
                self.gen_keyword_token("require ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Return { ref result, .. } => {
                self.gen_eager_expr_tokens(result, history, ExprTokenConfig::stmt())
            }
            FuncStmtVariant::Match { .. } => todo!(),
            FuncStmtVariant::ConditionFlow { .. } => panic!(),
        }
    }

    pub(crate) fn gen_func_branch_tokens(
        &mut self,
        stmt: &FuncStmt,
        branch: &FuncConditionFlowBranch,
        history: &Arc<History<'static>>,
    ) {
        match branch.variant {
            FuncConditionFlowBranchVariant::If { ref condition } => {
                self.gen_keyword_token("if ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.gen_special_token(":", None, None)
            }
            FuncConditionFlowBranchVariant::Elif { ref condition } => {
                self.gen_keyword_token("elif ", None, None);
                self.gen_eager_expr_tokens(condition, history, ExprTokenConfig::branch());
                self.gen_special_token(":", None, None)
            }
            FuncConditionFlowBranchVariant::Else => {
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
