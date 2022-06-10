use text::Text;
use vm::{History, VMControl};

use super::*;
use crate::*;

impl HuskyTraceTime {
    pub fn new_func_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<FuncStmt>,
        history: Arc<History<'static>>,
        text: &Text,
    ) -> Arc<Trace> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::FuncStmt { stmt, history },
            text,
        )
    }
    pub fn func_stmts_traces<'a>(
        &'a mut self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &'a [Arc<FuncStmt>],
        text: &'a Text,
        history: &'a Arc<History<'static>>,
    ) -> impl Iterator<Item = Arc<Trace>> + 'a {
        stmts.iter().map(move |stmt| {
            self.new_func_stmt_trace(parent_id, indent, stmt.clone(), history.clone(), text)
        })
    }

    pub(crate) fn func_stmt_lines(
        &mut self,
        stmt: &FuncStmt,
        text: &Text,
        history: &Arc<History<'static>>,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent: stmt.indent,
            tokens: self.func_stmt_tokens(stmt, text, history),
            idx: 0,
        }]
    }

    pub(crate) fn func_stmt_tokens(
        &mut self,
        stmt: &FuncStmt,
        text: &Text,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenProps> {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.ident.0));
                tokens.push(special!(" = "));
                tokens.extend(self.eager_expr_tokens(
                    initial_value,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            FuncStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![keyword!("assert ")];
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            FuncStmtVariant::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.eager_expr_tokens(
                    result,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            FuncStmtVariant::ConditionFlow { .. } => panic!(),
            FuncStmtVariant::ReturnXml { .. } => panic!(),
        }
    }

    pub(crate) fn func_stmt_figure(&self, stmt: &FuncStmt, history: &History) -> FigureProps {
        match stmt.variant {
            FuncStmtVariant::Init {
                varname,
                ref initial_value,
            } => todo!(),
            FuncStmtVariant::Assert { ref condition } => todo!(),
            FuncStmtVariant::Return { ref result } => todo!(),
            FuncStmtVariant::ConditionFlow { ref branches } => todo!(),
            FuncStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
            FuncStmtVariant::ReturnXml { ref xml_expr } => todo!(),
        }
    }
}