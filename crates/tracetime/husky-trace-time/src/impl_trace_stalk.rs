use crate::*;
use husky_entity_route::EntityRoutePtr;

impl HuskyTracetime {
    pub fn keyed_trace_stalk(&mut self, trace_id: TraceId) -> (TraceStalkKey, TraceStalk) {
        let sample_id = self.restriction.opt_sample_id().unwrap();
        let key = TraceStalkKey::from_trace_data(sample_id, &self.trace(trace_id).raw_data);
        if !self.trace_stalks.contains_key(&key) {
            self.trace_stalks
                .insert(key.clone(), self.produce_trace_stalk(trace_id, sample_id));
        }
        let trace_stalk_raw_data = self.trace_stalks[&key].clone();
        (key, trace_stalk_raw_data)
    }

    fn produce_trace_stalk(&self, trace_id: TraceId, sample_id: SampleId) -> TraceStalk {
        let trace: &Trace = self.trace(trace_id);
        match trace.variant {
            TraceVariant::Main(ref repr) => self.trace_stalk_from_result(
                self.runtime().eval_feature_repr_cached(repr, sample_id),
                repr.ty(),
            ),
            TraceVariant::EntityFeature { ref repr, .. } => self.trace_stalk_from_result(
                self.runtime().eval_feature_repr_cached(repr, sample_id),
                repr.ty(),
            ),
            TraceVariant::FeatureStmt(ref stmt) => match stmt.variant {
                FeatureLazyStmtVariant::Init { ref value, .. } => {
                    self.trace_stalk_from_expr(value, sample_id)
                }
                FeatureLazyStmtVariant::Assert { ref condition } => {
                    self.trace_stalk_from_expr(condition, sample_id)
                }
                FeatureLazyStmtVariant::Require { ref condition, .. } => {
                    self.trace_stalk_from_expr(condition, sample_id)
                }
                FeatureLazyStmtVariant::Return { ref result }
                | FeatureLazyStmtVariant::ReturnUnveil { ref result, .. } => {
                    self.trace_stalk_from_expr(result, sample_id)
                }
                FeatureLazyStmtVariant::ConditionFlow { .. } => panic!(),
                FeatureLazyStmtVariant::ReturnXml { .. } => todo!(),
            },
            TraceVariant::FeatureBranch(_) => Default::default(),
            TraceVariant::FeatureExpr(ref expr) => self.trace_stalk_from_expr(expr, sample_id),
            TraceVariant::FeatureCallArgument { ref argument, .. } => {
                self.trace_stalk_from_expr(argument, sample_id)
            }
            TraceVariant::Module { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::EagerExpr { .. }
            | TraceVariant::CallHead { .. }
            | TraceVariant::FuncBranch { .. }
            | TraceVariant::ProcBranch { .. }
            | TraceVariant::LoopFrame { .. }
            | TraceVariant::EagerCallArgument { .. } => TraceStalk::default(),
        }
    }

    pub(crate) fn update_trace_stalks(&mut self) -> Vec<(TraceStalkKey, TraceStalk)> {
        if let Some(sample_id) = self.restriction.opt_sample_id() {
            let mut trace_stalks = Vec::new();
            for root_trace_id in self.root_trace_ids.clone() {
                self.collect_new_trace_stalks_within_trace(
                    sample_id,
                    root_trace_id,
                    &mut trace_stalks,
                );
            }
            trace_stalks
        } else {
            vec![]
        }
    }

    fn collect_new_trace_stalks_within_trace(
        &mut self,
        sample_id: SampleId,
        trace_id: TraceId,
        trace_stalks: &mut Vec<(TraceStalkKey, TraceStalk)>,
    ) {
        let trace_node_data = self.trace_node_data(trace_id);
        let expanded = trace_node_data.expanded;
        let trace_raw_data = &trace_node_data.trace_data;
        let trace_stalk_key = TraceStalkKey::from_trace_data(sample_id, trace_raw_data);
        let associated_trace_ids = trace_raw_data.associated_trace_ids();
        if !self.trace_stalks.contains_key(&trace_stalk_key) {
            let (key, data) = self.keyed_trace_stalk(trace_id);
            self.trace_stalks.insert(key.clone(), data.clone());
            trace_stalks.push((key, data))
        }
        for associated_trace_id in associated_trace_ids {
            self.collect_new_trace_stalks_within_trace(sample_id, associated_trace_id, trace_stalks)
        }
        if expanded {
            for subtrace_id in self.subtrace_ids(trace_id) {
                self.collect_new_trace_stalks_within_trace(sample_id, subtrace_id, trace_stalks)
            }
        }
    }

    fn trace_stalk_from_expr(&self, expr: &FeatureLazyExpr, sample_id: SampleId) -> TraceStalk {
        let arrived = match self
            .runtime
            .eval_opt_arrival_indicator_cached(expr.opt_arrival_indicator.as_ref(), sample_id)
        {
            Ok(arrived) => arrived,
            Err(e) => false,
        };
        if arrived {
            self.trace_stalk_from_result(
                self.runtime().eval_feature_expr(expr, sample_id),
                expr.expr.intrinsic_ty(),
            )
        } else {
            TraceStalk::unarrived()
        }
    }

    fn trace_stalk_from_result(
        &self,
        result: __VMResult<__Register<'static>>,
        ty: EntityRoutePtr,
    ) -> TraceStalk {
        TraceStalk {
            extra_tokens: vec![
                TraceTokenData {
                    kind: TraceTokenKind::Fade,
                    value: " = ".to_string(),
                    opt_associated_trace_id: None,
                },
                self.trace_token_from_result(result, ty),
            ],
            kind: TraceStalkKind::Value,
        }
    }

    pub(crate) fn trace_token_from_result(
        &self,
        result: __VMResult<__Register<'static>>,
        ty: EntityRoutePtr,
    ) -> TraceTokenData {
        match result {
            Ok(value) => self.trace_token_from_value(value, ty),
            Err(e) => TraceTokenData {
                kind: TraceTokenKind::Error,
                value: e.message().to_string(),
                opt_associated_trace_id: None,
            },
        }
    }

    pub(crate) fn trace_token_from_value(
        &self,
        value: __Register<'static>,
        ty: EntityRoutePtr,
    ) -> TraceTokenData {
        TraceTokenData {
            kind: TraceTokenKind::Fade,
            value: self.comptime().print_short(&value, ty),
            opt_associated_trace_id: None,
        }
    }
}
