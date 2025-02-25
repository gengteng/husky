use husky_vm::History;

use super::*;
use crate::*;

impl HuskyTracetime {
    pub(crate) fn func_stmt_figure(
        &self,
        stmt: &FuncStmt,
        history: &History<'static>,
    ) -> FigureCanvasData {
        match stmt.variant {
            FuncStmtVariant::Init {
                ref initial_value, ..
            } => self.eager_expr_figure(initial_value, history),
            FuncStmtVariant::Require { .. } | FuncStmtVariant::Assert { .. } => {
                FigureCanvasData::void()
            }
            FuncStmtVariant::Return { ref result, .. } => self.eager_expr_figure(result, history),
            FuncStmtVariant::ConditionFlow { .. } => todo!(),
            FuncStmtVariant::Match { .. } => todo!(),
        }
    }
}
