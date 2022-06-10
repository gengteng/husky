use crate::*;

use super::{impl_expr::ExprTokenConfig, *};

impl HuskyTraceTime {
    pub fn feature_stmt_traces(
        &mut self,
        parent: &Trace,
        stmt: Arc<FeatureStmt>,
        text: &Text,
    ) -> Vec<Arc<Trace>> {
        match stmt.variant {
            FeatureStmtVariant::Init { .. }
            | FeatureStmtVariant::Assert { .. }
            | FeatureStmtVariant::Return { .. } => {
                vec![self.new_trace(
                    Some(parent.id()),
                    stmt.indent,
                    TraceVariant::FeatureStmt(stmt),
                    text,
                )]
            }
            FeatureStmtVariant::ConditionFlow { ref branches, .. } => branches
                .iter()
                .map(|branch| self.feature_branch_trace(parent, stmt.indent, branch.clone(), text))
                .collect(),
        }
    }

    pub fn feature_stmt_lines(&mut self, stmt: &FeatureStmt, text: &Text) -> Vec<LineProps> {
        vec![LineProps {
            indent: stmt.indent,
            idx: 0,
            tokens: self.feature_stmt_tokens(stmt, text),
        }]
    }

    pub fn feature_stmt_tokens(&mut self, stmt: &FeatureStmt, text: &Text) -> Vec<TraceTokenProps> {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => {
                let mut tokens = vec![];
                tokens.push(ident!(varname.0));
                tokens.push(special!(" = "));
                tokens.extend(self.feature_expr_tokens(value, text, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![];
                tokens.push(keyword!("assert "));
                tokens.extend(self.feature_expr_tokens(condition, text, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::Return { ref result } => {
                let mut tokens = vec![];
                tokens.extend(self.feature_expr_tokens(result, text, ExprTokenConfig::stmt()));
                tokens
            }
            FeatureStmtVariant::ConditionFlow { .. } => panic!(),
        }
    }
    pub(crate) fn feature_stmt_figure(&self, stmt: &FeatureStmt, focus: &Focus) -> FigureProps {
        match stmt.variant {
            FeatureStmtVariant::Init { varname, ref value } => {
                self.feature_expr_figure(value, focus)
            }
            FeatureStmtVariant::Assert { .. } => FigureProps::void(),
            FeatureStmtVariant::Return { ref result } => self.feature_expr_figure(result, focus),
            FeatureStmtVariant::ConditionFlow { ref branches } => FigureProps::void(),
        }
    }
}