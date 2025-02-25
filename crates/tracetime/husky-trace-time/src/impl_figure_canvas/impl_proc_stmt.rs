use super::*;

impl HuskyTracetime {
    pub(crate) fn proc_stmt_figure(
        &self,
        stmt: &ProcStmt,
        history: &History<'static>,
    ) -> FigureCanvasData {
        match stmt.variant {
            ProcStmtVariant::Init {
                ref initial_value, ..
            } => self.eager_expr_figure(initial_value, history),
            ProcStmtVariant::Assert { .. } => todo!(),
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
                    FigureCanvasData::void()
                }
            }
            ProcStmtVariant::Return { ref result, .. } => self.eager_expr_figure(result, history),
            ProcStmtVariant::ConditionFlow { .. } => todo!(),
            ProcStmtVariant::Loop { .. } => {
                if let Some(entry) = history.get(stmt) {
                    match entry {
                        HistoryEntry::Loop { ref mutations, .. } => {
                            self.mutations_figure(mutations)
                        }
                        _ => panic!(),
                    }
                } else {
                    FigureCanvasData::void()
                }
            }
            ProcStmtVariant::Break => FigureCanvasData::void(),
            ProcStmtVariant::Match { .. } => todo!(),
        }
    }

    pub fn loop_frame_mutations_figure(
        &self,
        loop_trace_id: TraceId,
        frame_mutations: &[MutationData],
    ) -> FigureCanvasData {
        let loop_trace = self.trace(loop_trace_id);
        let mutations = match loop_trace.variant {
            TraceVariant::ProcStmt {
                ref stmt,
                ref history,
            } => match history.get(stmt).unwrap() {
                HistoryEntry::Loop {
                    stack_snapshot,
                    mutations,
                    ..
                } => mutations
                    .iter()
                    .enumerate()
                    .map(|(idx, mutation_data)| {
                        if frame_mutations
                            .iter()
                            .find(|frame_mutation| {
                                frame_mutation.varidx() == mutation_data.varidx()
                            })
                            .is_some()
                        {
                            self.mutation_figure(idx, mutation_data)
                        } else {
                            MutationFigureData {
                                name: match mutation_data.kind {
                                    MutationDataVariant::Exec => panic!(),
                                    MutationDataVariant::Block { varname, .. } => {
                                        varname.as_str().to_string()
                                    }
                                },
                                before: None,
                                after: FigureCanvasData::new_specific(
                                    self.visualize_temp_value(
                                        &stack_snapshot[mutation_data.varidx()].snapshot(),
                                        mutation_data.ty,
                                        mutation_data.file,
                                        mutation_data.range,
                                    )
                                    .unwrap(),
                                ),
                                idx,
                            }
                        }
                    })
                    .collect(),
                _ => panic!(),
            },
            _ => panic!(),
        };
        FigureCanvasData::Mutations { mutations }
    }

    pub fn mutations_figure(&self, mutations: &[MutationData<'static>]) -> FigureCanvasData {
        FigureCanvasData::Mutations {
            mutations: mutations
                .iter()
                .enumerate()
                .map(|(idx, mutation)| self.mutation_figure(idx, mutation))
                .collect(),
        }
    }

    fn mutation_figure(
        &self,
        idx: usize,
        mutation_data: &MutationData<'static>,
    ) -> MutationFigureData {
        MutationFigureData {
            name: match mutation_data.kind {
                MutationDataVariant::Exec => {
                    let text = self.runtime().comptime().text(mutation_data.file).unwrap();
                    text.ranged(mutation_data.range)
                }
                MutationDataVariant::Block { varname, .. } => varname.as_str().to_string(),
            },
            before: if let Some(before) = mutation_data.before.as_ref() {
                Some(FigureCanvasData::new_specific(
                    self.visualize_temp_value(
                        before,
                        mutation_data.ty,
                        mutation_data.file,
                        mutation_data.range,
                    )
                    .unwrap(),
                ))
            } else {
                None
            },
            after: FigureCanvasData::new_specific(
                self.visualize_temp_value(
                    &mutation_data.after,
                    mutation_data.ty,
                    mutation_data.file,
                    mutation_data.range,
                )
                .unwrap(),
            )
            .into(),
            idx,
        }
    }
}
