use entity_route::RangedEntityRoute;
use text::Text;
use vm::{History, StackValueSnapshot};

use super::{impl_expr::ExprTokenConfig, *};
use crate::*;

impl HuskyTraceTime {
    pub fn new_eager_expr_trace(
        &mut self,
        text: &Text,
        expr: Arc<EagerExpr>,
        history: Arc<History<'static>>,
        opt_parent: Option<&Trace>,
        indent: Indent,
    ) -> Arc<Trace> {
        self.new_trace(
            opt_parent.map(|parent| parent.id()),
            indent,
            TraceVariant::EagerExpr { expr, history },
            text,
        )
    }

    pub(crate) fn eager_expr_lines(
        &mut self,
        text: &Text,
        expr: &Arc<EagerExpr>,
        history: &Arc<History<'static>>,
        indent: u8,
        config: ExprTokenConfig,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent,
            idx: 0,
            tokens: self.eager_expr_tokens(expr, text, history, config),
        }]
    }

    pub(crate) fn eager_expr_tokens(
        &mut self,
        expr: &Arc<EagerExpr>,
        text: &Text,
        history: &Arc<History<'static>>,
        config: ExprTokenConfig,
    ) -> Vec<TraceTokenProps> {
        let associated_trace_id = if config.associated {
            Some(
                self.new_eager_expr_trace(text, expr.clone(), history.clone(), None, 0)
                    .id(),
            )
        } else {
            None
        };
        let mut tokens = vec![];
        match expr.variant {
            EagerExprVariant::Variable { varname, .. } => {
                tokens.push(ident!(varname.0, associated_trace_id))
            }
            EagerExprVariant::EntityRoute { route: scope } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => return vec![literal!(value)],
            EagerExprVariant::Bracketed(ref expr) => {
                tokens.push(special!("("));
                tokens.extend(self.eager_expr_tokens(expr, text, history, config.subexpr()));
                tokens.push(special!(")"));
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => match opn_variant {
                EagerOpnVariant::Binary { opr, this_ty: this } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!(opr.spaced_code(), associated_trace_id));
                    tokens.extend(self.eager_expr_tokens(
                        &opds[1],
                        text,
                        history,
                        config.subexpr(),
                    ));
                }
                EagerOpnVariant::Prefix { opr, .. } => {
                    tokens.push(special!(opr.code(), associated_trace_id));
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                }
                EagerOpnVariant::Suffix { opr, .. } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!(opr.code(), associated_trace_id));
                }
                EagerOpnVariant::RoutineCall(ranged_scope) => {
                    tokens = self.eager_routine_call_tokens(
                        *ranged_scope,
                        opds,
                        associated_trace_id,
                        text,
                        history,
                        &config,
                    )
                }
                EagerOpnVariant::FieldAccess { field_ident, .. } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!("."));
                    tokens.push(ident!(field_ident.ident.0));
                }
                EagerOpnVariant::MethodCall { method_ident, .. } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!("."));
                    tokens.push(ident!(method_ident.ident.0));
                    tokens.push(special!("("));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(
                            &opds[i],
                            text,
                            history,
                            config.subexpr(),
                        ));
                    }
                    tokens.push(special!(")"));
                }
                EagerOpnVariant::ElementAccess { element_binding } => {
                    tokens.extend(self.eager_expr_tokens(
                        &opds[0],
                        text,
                        history,
                        config.subexpr(),
                    ));
                    tokens.push(special!("[", associated_trace_id.clone()));
                    for i in 1..opds.len() {
                        if i > 1 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(
                            &opds[i],
                            text,
                            history,
                            config.subexpr(),
                        ));
                    }
                    tokens.push(special!("]", associated_trace_id));
                }
                EagerOpnVariant::TypeCall { ranged_ty, .. } => {
                    tokens.push(route!(text.ranged(ranged_ty.range)));
                    tokens.push(special!("("));
                    for i in 0..opds.len() {
                        if i > 0 {
                            tokens.push(special!(", "))
                        }
                        tokens.extend(self.eager_expr_tokens(
                            &opds[i],
                            text,
                            history,
                            config.subexpr(),
                        ));
                    }
                    tokens.push(special!(")"));
                }
            },
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { .. } => todo!(),
            EagerExprVariant::ThisField { .. } => todo!(),
            EagerExprVariant::EnumKindLiteral(_) => todo!(),
        };
        if config.appended {
            tokens.push(fade!(" = "));
            tokens.push(history.value(expr).into())
        }
        tokens
    }

    fn eager_routine_call_tokens(
        &mut self,
        ranged_scope: RangedEntityRoute,
        inputs: &[Arc<EagerExpr>],
        opt_associated_trace_id: Option<TraceId>,
        text: &Text,
        history: &Arc<History<'static>>,
        config: &ExprTokenConfig,
    ) -> Vec<TraceTokenProps> {
        let mut tokens = vec![
            route!(text.ranged(ranged_scope.range), opt_associated_trace_id),
            special!("("),
        ];
        for (i, input) in inputs.iter().enumerate() {
            if i > 0 {
                tokens.push(special!(", "));
            }
            tokens.extend(self.eager_expr_tokens(input, text, history, config.subexpr()));
        }
        tokens.push(special!(")"));
        tokens
    }

    pub(crate) fn eager_expr_figure(&self, expr: &EagerExpr, history: &History) -> FigureProps {
        if let Some(entry) = history.get(expr) {
            match entry {
                HistoryEntry::PureExpr { output } => match output {
                    Ok(output) => {
                        let visual_props = self.runtime.visualize(expr.ty(), output.any_ref());
                        FigureProps::new_specific(visual_props)
                    }
                    Err(e) => FigureProps::void(),
                },
                HistoryEntry::Exec { .. } => todo!(),
                HistoryEntry::Loop { .. } => panic!(),
                HistoryEntry::ControlFlow {
                    opt_branch_entered: enter,
                    ..
                } => todo!(),
                HistoryEntry::Break => todo!(),
                HistoryEntry::PatternMatching { .. } => todo!(),
            }
        } else {
            FigureProps::void()
        }
    }
}