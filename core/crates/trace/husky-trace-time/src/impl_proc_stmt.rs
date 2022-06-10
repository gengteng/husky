use husky_compile_time::HuskyCompileTime;
use print_utils::{epin, p};
use text::Text;
use upcast::Upcast;
use vm::*;

use super::{impl_expr::ExprTokenConfig, *};
use crate::*;

impl HuskyTraceTime {
    fn new_proc_stmt_trace(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        text: &Text,
        history: Arc<History<'static>>,
    ) -> Arc<Trace> {
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::ProcStmt { stmt, history },
            text,
        )
    }

    fn new_proc_branch_trace(
        &mut self,
        text: &Text,
        parent_id: TraceId,
        indent: Indent,
        stmt: Arc<ProcStmt>,
        branch: Arc<ProcConditionBranch>,
        branch_idx: u8,
        history: Arc<History<'static>>,
    ) -> Arc<Trace> {
        let opt_vm_branch = history.get(&stmt).map(|entry| match entry {
            HistoryEntry::ControlFlow { vm_branches, .. } => {
                vm_branches[branch_idx as usize].clone()
            }
            _ => panic!(),
        });
        self.new_trace(
            Some(parent_id),
            indent,
            TraceVariant::ProcBranch {
                stmt,
                branch,
                branch_idx,
                opt_vm_branch,
                history,
            },
            text,
        )
    }

    pub(crate) fn proc_stmt_lines(
        &mut self,
        stmt: &ProcStmt,
        text: &Text,
        history: &Arc<History<'static>>,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent: stmt.indent,
            tokens: self.proc_stmt_tokens(stmt, text, history),
            idx: 0,
        }]
    }

    pub(crate) fn proc_stmt_tokens(
        &mut self,
        stmt: &ProcStmt,
        text: &Text,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenProps> {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => {
                let mut tokens = vec![keyword!(match init_kind {
                    vm::InitKind::Let => "let ",
                    vm::InitKind::Var => "var ",
                    vm::InitKind::Decl => panic!(),
                })];
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
            ProcStmtVariant::Assert { ref condition } => {
                let mut tokens = vec![keyword!("assert ")];
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ProcStmtVariant::Execute { ref expr } => {
                let mut tokens =
                    self.eager_expr_tokens(expr, text, history, ExprTokenConfig::exec());
                match expr.variant {
                    EagerExprVariant::Opn {
                        ref opn_variant, ..
                    } => match opn_variant {
                        EagerOpnVariant::Binary { opr, this_ty: this } => match opr {
                            BinaryOpr::Assign(_) => {
                                tokens.push(fade!(" = "));
                                tokens.push(history.value(expr).into())
                            }
                            BinaryOpr::Pure(_) => (),
                        },
                        _ => (),
                    },
                    _ => panic!(),
                }
                tokens
            }
            ProcStmtVariant::Return { ref result } => {
                let mut tokens = vec![keyword!("return ")];
                tokens.extend(self.eager_expr_tokens(
                    result,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            ProcStmtVariant::ConditionFlow { ref branches } => todo!(),
            ProcStmtVariant::Loop {
                loop_variant: ref loop_kind,
                ref stmts,
            } => match loop_kind {
                LoopVariant::For {
                    frame_var,
                    ref initial_boundary,
                    ref final_boundary,
                    ..
                } => {
                    let mut tokens = vec![keyword!("for ")];
                    tokens.extend(self.initial_boundary_tokens(initial_boundary, text, history));
                    tokens.push(ident!(frame_var.ident.0));
                    tokens.extend(self.final_boundary_tokens(final_boundary, text, history));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopVariant::ForExt {
                    frame_var,
                    ref final_boundary,
                    ..
                } => {
                    let mut tokens = vec![keyword!("forext ")];
                    tokens.push(ident!(frame_var.ident.0));
                    tokens.extend(self.final_boundary_tokens(final_boundary, text, history));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopVariant::While { ref condition } => {
                    let mut tokens = vec![keyword!("while ")];
                    tokens.extend(self.eager_expr_tokens(
                        condition,
                        text,
                        history,
                        ExprTokenConfig::loop_head(),
                    ));
                    tokens.push(special!(":"));
                    tokens
                }
                LoopVariant::DoWhile { condition } => {
                    let mut tokens = vec![keyword!("do while ")];
                    tokens.extend(self.eager_expr_tokens(
                        condition,
                        text,
                        history,
                        ExprTokenConfig::loop_head(),
                    ));
                    tokens.push(special!(":"));
                    tokens
                }
            },
            ProcStmtVariant::Break => vec![keyword!("break")],
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }

    fn initial_boundary_tokens(
        &mut self,
        boundary: &Boundary,
        text: &Text,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenProps> {
        match boundary.opt_bound {
            Some(ref bound) => {
                let mut tokens =
                    self.eager_expr_tokens(bound, text, history, ExprTokenConfig::stmt());
                match boundary.kind {
                    BoundaryKind::UpperOpen => tokens.push(special!(" > ")),
                    BoundaryKind::UpperClosed => tokens.push(special!(" >= ")),
                    BoundaryKind::LowerOpen => tokens.push(special!(" < ")),
                    BoundaryKind::LowerClosed => tokens.push(special!(" <= ")),
                }
                tokens
            }
            None => vec![],
        }
    }

    fn final_boundary_tokens(
        &mut self,
        boundary: &Boundary,
        text: &Text,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenProps> {
        match boundary.opt_bound {
            Some(ref bound) => {
                let mut tokens = vec![special!(match boundary.kind {
                    BoundaryKind::UpperOpen => " < ",
                    BoundaryKind::UpperClosed => " <= ",
                    BoundaryKind::LowerOpen => " > ",
                    BoundaryKind::LowerClosed => " >= ",
                })];
                tokens.extend(self.eager_expr_tokens(
                    bound,
                    text,
                    history,
                    ExprTokenConfig::stmt(),
                ));
                tokens
            }
            None => vec![],
        }
    }

    pub fn proc_stmts_traces(
        &mut self,
        parent_id: TraceId,
        indent: Indent,
        stmts: &[Arc<ProcStmt>],
        text: &Text,
        history: &Arc<History<'static>>,
    ) -> Vec<Arc<Trace>> {
        let mut traces = Vec::new();
        for stmt in stmts {
            match stmt.variant {
                ProcStmtVariant::ConditionFlow { ref branches } => {
                    for (branch_idx, branch) in branches.iter().enumerate() {
                        traces.push(self.new_proc_branch_trace(
                            text,
                            parent_id,
                            indent,
                            stmt.clone(),
                            branch.clone(),
                            branch_idx.try_into().unwrap(),
                            history.clone(),
                        ))
                    }
                }
                _ => traces.push(self.new_proc_stmt_trace(
                    parent_id,
                    indent,
                    stmt.clone(),
                    text,
                    history.clone(),
                )),
            }
        }
        traces
    }

    pub(crate) fn loop_subtraces(
        &mut self,
        db: &dyn EvalFeature<'static>,
        parent: &Trace,
        loop_kind: VMLoopKind,
        loop_stmt: &Arc<ProcStmt>,
        body_stmts: &Arc<Vec<Arc<ProcStmt>>>,
        stack_snapshot: &StackSnapshot<'static>,
        body_instruction_sheet: &Arc<InstructionSheet>,
        verbose: bool,
    ) -> Arc<Vec<Arc<Trace>>> {
        let text = db.compile_time().text(parent.file).unwrap();
        let frames = exec_loop_debug(
            db.upcast(),
            loop_kind,
            &body_instruction_sheet,
            stack_snapshot,
            verbose,
        );
        Arc::new(
            frames
                .into_iter()
                .map(|loop_frame_data| {
                    self.new_trace(
                        Some(parent.id()),
                        parent.props.indent + 2,
                        TraceVariant::LoopFrame {
                            loop_stmt: loop_stmt.clone(),
                            body_stmts: body_stmts.clone(),
                            body_instruction_sheet: body_instruction_sheet.clone(),
                            loop_frame_data,
                        },
                        &text,
                    )
                })
                .collect(),
        )
    }

    pub(crate) fn loop_frame_subtraces(
        &mut self,
        db: &dyn EvalFeature<'static>,
        text: &Text,
        loop_stmt: &Arc<ProcStmt>,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        loop_frame_data: &LoopFrameData<'static>,
        parent: &Trace,
        verbose: bool,
    ) -> Avec<Trace> {
        let history = exec_debug(
            db.upcast(),
            instruction_sheet,
            &loop_frame_data.stack_snapshot,
            verbose,
        );
        let mut subtraces: Vec<_> =
            self.proc_stmts_traces(parent.id(), parent.props.indent + 2, stmts, text, &history);
        match loop_stmt.variant {
            ProcStmtVariant::Loop {
                ref loop_variant, ..
            } => match loop_variant {
                LoopVariant::For { .. } | LoopVariant::ForExt { .. } => (),
                LoopVariant::While { condition } => subtraces.insert(
                    0,
                    self.new_eager_expr_trace(
                        text,
                        condition.clone(),
                        history.clone(),
                        Some(parent),
                        parent.props.indent + 2,
                    ),
                ),
                LoopVariant::DoWhile { condition } => subtraces.push(self.new_eager_expr_trace(
                    text,
                    condition.clone(),
                    history.clone(),
                    Some(parent),
                    parent.props.indent + 2,
                )),
            },
            _ => panic!(),
        }
        Arc::new(subtraces)
    }

    pub(crate) fn loop_frame_lines(
        &self,
        indent: Indent,
        loop_frame_data: &LoopFrameData,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent,
            tokens: self.loop_frame_tokens(loop_frame_data),
            idx: 0,
        }]
    }

    pub(crate) fn loop_frame_tokens(&self, vm_loop_frame: &LoopFrameData) -> Vec<TraceTokenProps> {
        match vm_loop_frame.frame_kind {
            vm::FrameKind::For(frame_var) => {
                vec![
                    keyword!("frame "),
                    ident!(frame_var.0),
                    special!(" = "),
                    literal!(format!("{}", vm_loop_frame.frame_var_value)),
                ]
            }
            vm::FrameKind::Loop => {
                vec![
                    keyword!("frame "),
                    literal!(format!("{}", vm_loop_frame.frame_var_value)),
                ]
            }
        }
    }

    pub(crate) fn proc_branch_subtraces(
        &mut self,
        db: &dyn EvalFeature<'static>,
        text: &Text,
        stmts: &[Arc<ProcStmt>],
        instruction_sheet: &InstructionSheet,
        stack_snapshot: &StackSnapshot<'static>,
        parent: &Trace,
        verbose: bool,
    ) -> Avec<Trace> {
        let history = exec_debug(db.upcast(), instruction_sheet, stack_snapshot, verbose);
        Arc::new(self.proc_stmts_traces(
            parent.id(),
            parent.props.indent + 2,
            stmts,
            text,
            &history,
        ))
    }

    pub(crate) fn proc_branch_lines(
        &mut self,
        text: &Text,
        indent: Indent,
        branch: &ProcConditionBranch,
        history: &Arc<History<'static>>,
    ) -> Vec<LineProps> {
        vec![LineProps {
            indent,
            tokens: self.proc_branch_tokens(text, indent, branch, history),
            idx: 0,
        }]
    }

    pub(crate) fn proc_branch_tokens(
        &mut self,
        text: &Text,
        indent: Indent,
        branch: &ProcConditionBranch,
        history: &Arc<History<'static>>,
    ) -> Vec<TraceTokenProps> {
        let mut tokens = Vec::new();
        match branch.variant {
            ProcConditionBranchVariant::If { ref condition } => {
                tokens.push(keyword!("if "));
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::branch(),
                ));
                tokens.push(special!(":"))
            }
            ProcConditionBranchVariant::Elif { ref condition } => {
                tokens.push(keyword!("elif "));
                tokens.extend(self.eager_expr_tokens(
                    condition,
                    text,
                    history,
                    ExprTokenConfig::branch(),
                ));
                tokens.push(special!(":"))
            }
            ProcConditionBranchVariant::Else => {
                tokens.push(keyword!("else"));
                tokens.push(special!(":"))
            }
        }
        tokens
    }

    pub(crate) fn proc_stmt_figure(&self, stmt: &ProcStmt, history: &History) -> FigureProps {
        match stmt.variant {
            ProcStmtVariant::Init {
                varname,
                ref initial_value,
                init_kind,
            } => self.eager_expr_figure(initial_value, history),
            ProcStmtVariant::Assert { ref condition } => todo!(),
            ProcStmtVariant::Execute { ref expr } => {
                if let Some(entry) = history.get(expr) {
                    match entry {
                        HistoryEntry::Exec { ref mutations } => self.mutations_figure(mutations),
                        _ => {
                            p!(entry);
                            panic!("wrong kind of entry")
                        }
                    }
                } else {
                    FigureProps::void()
                }
            }
            ProcStmtVariant::Return { ref result } => self.eager_expr_figure(result, history),
            ProcStmtVariant::ConditionFlow { ref branches } => todo!(),
            ProcStmtVariant::Loop {
                ref loop_variant,
                ref stmts,
            } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::Loop { ref mutations, .. } => {
                            self.mutations_figure(mutations)
                        }
                        _ => panic!(),
                    }
                } else {
                    FigureProps::void()
                }
            }
            ProcStmtVariant::Break => FigureProps::void(),
            ProcStmtVariant::Match {
                ref match_expr,
                ref branches,
            } => todo!(),
        }
    }

    pub fn loop_frame_mutations_figure(
        &self,
        loop_trace_id: TraceId,
        frame_mutations: &[MutationData],
        frame_stack_snapshot: &StackSnapshot,
    ) -> FigureProps {
        let loop_trace = self.trace(loop_trace_id);
        let mutations = match loop_trace.variant {
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => match history.get(stmt).unwrap() {
                HistoryEntry::Loop {
                    loop_kind,
                    control,
                    stack_snapshot,
                    body_instruction_sheet: body,
                    mutations,
                } => mutations
                    .iter()
                    .enumerate()
                    .map(|(idx, mutation)| {
                        if let Some(frame_mutation) = frame_mutations
                            .iter()
                            .find(|frame_mutation| frame_mutation.varidx() == mutation.varidx())
                        {
                            todo!()
                            // MutationFigureProps::new(
                            //     self,
                            //     &self.compile_time().text(frame_mutation.file).unwrap(),
                            //     &self.visualizer(frame_mutation.ty),
                            //     frame_mutation,
                            //     idx,
                            //     self.verbose(),
                            // )
                        } else {
                            MutationFigureProps {
                                name: match mutation.kind {
                                    MutationDataKind::Exec { range } => panic!(),
                                    MutationDataKind::Block { stack_idx, varname } => {
                                        varname.as_str().to_string()
                                    }
                                },
                                before: None,
                                after: FigureProps::new_specific(self.runtime.visualize(
                                    mutation.ty,
                                    frame_stack_snapshot[mutation.varidx()].any_ref(),
                                )),
                                idx,
                            }
                        }
                    })
                    .collect(),
                _ => panic!(),
            },
            _ => panic!(),
        };
        FigureProps::Mutations { mutations }
    }

    pub fn mutations_figure(&self, mutations: &[MutationData]) -> FigureProps {
        FigureProps::Mutations {
            mutations: mutations
                .iter()
                .enumerate()
                .map(|(i, mutation)| {
                    todo!()
                    // MutationFigureProps::new(
                    //     self,
                    //     &self.compile_time().text(mutation.file).unwrap(),
                    //     &self.visualizer(mutation.ty),
                    //     mutation,
                    //     i,
                    //     self.verbose(),
                    // )
                })
                .collect(),
        }
    }
}