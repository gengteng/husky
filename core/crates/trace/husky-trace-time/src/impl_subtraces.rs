use check_utils::should_eq;

use crate::*;

impl HuskyTraceTime {
    pub fn gen_subtraces(&mut self, trace_id: TraceId) -> Vec<TraceId> {
        let trace = unsafe { self.trace_ref(trace_id) };
        match trace.variant {
            TraceVariant::Main(ref repr) => self.feature_repr_subtraces(&trace, repr),
            TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureCallInput { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::CallHead { .. } => vec![],
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => match stmt.variant {
                ProcStmtVariant::Init { .. }
                | ProcStmtVariant::Assert { .. }
                | ProcStmtVariant::Execute { .. }
                | ProcStmtVariant::Return { .. } => vec![],
                ProcStmtVariant::ConditionFlow { .. } => panic!(),
                ProcStmtVariant::Loop { ref stmts, .. } => {
                    match history
                        .get(stmt)
                        .expect("if there is no entry, there is no subtraces")
                    {
                        HistoryEntry::PureExpr { .. } | HistoryEntry::Exec { .. } => {
                            vec![]
                        }
                        HistoryEntry::Loop {
                            control,
                            ref stack_snapshot,
                            body_instruction_sheet: ref body,
                            loop_kind,
                            ..
                        } => self.loop_subtraces(
                            trace,
                            *loop_kind,
                            stmt,
                            stmts,
                            stack_snapshot,
                            body,
                            self.runtime.verbose(),
                        ),
                        HistoryEntry::ControlFlow {
                            opt_branch_entered: enter,
                            ..
                        } => todo!(),
                        HistoryEntry::Break => todo!(),
                        HistoryEntry::PatternMatching { .. } => todo!(),
                    }
                }
                ProcStmtVariant::Break => vec![],
                ProcStmtVariant::Match {
                    ref match_expr,
                    ref branches,
                } => todo!(),
            },
            TraceVariant::FeatureExpr(ref expr) => {
                todo!()
                // self.feature_expr_subtraces(trace, expr, effective_opt_input_id)
            }
            TraceVariant::FeatureBranch(ref branch) => {
                self.feature_lazy_block_subtraces(trace, &branch.block)
            }
            TraceVariant::EagerExpr {
                ref expr,
                ref history,
            } => {
                todo!() // self.eager_expr_subtraces(trace, expr, history)
            }
            TraceVariant::LoopFrame {
                ref loop_frame_data,
                ref loop_stmt,
                ref body_stmts,
                ref body_instruction_sheet,
            } => self.loop_frame_subtraces(
                loop_stmt,
                body_stmts,
                body_instruction_sheet,
                loop_frame_data,
                trace,
            ),
            TraceVariant::ProcBranch {
                ref stmt,
                branch_idx,
                ref history,
                ref opt_vm_branch,
                ref branch,
                ..
            } => match history.get(stmt).unwrap() {
                HistoryEntry::ControlFlow {
                    stack_snapshot,
                    opt_branch_entered: branch_entered,
                    ..
                } => {
                    todo!()
                    // should_eq!(Some(branch_idx), *branch_entered);
                    // self.proc_branch_subtraces(
                    //     &branch.stmts,
                    //     &opt_vm_branch.as_ref().unwrap().body,
                    //     stack_snapshot,
                    //     trace,
                    // )
                }
                _ => panic!(),
            },
        }
    }
}