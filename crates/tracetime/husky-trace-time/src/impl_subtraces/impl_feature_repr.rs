use crate::*;
use husky_feature_eval::FeatureEvaluator;

impl HuskyTracetime {
    pub fn feature_repr_subtraces(
        &mut self,
        parent: &Trace,
        feature_repr: &FeatureRepr,
    ) -> Option<Vec<TraceId>> {
        match feature_repr {
            FeatureRepr::Value { .. } => todo!(),
            FeatureRepr::LazyExpr(_) => todo!(),
            FeatureRepr::LazyBlock(block) => Some(self.feature_lazy_block_subtraces(parent, block)),
            FeatureRepr::FuncBlock(block) => self.feature_func_block_subtraces(parent, block),
            FeatureRepr::ProcBlock(block) => self.feature_proc_block_subtraces(parent, block),
            FeatureRepr::TargetInput { .. } => todo!(),
        }
    }

    pub fn feature_lazy_block_subtraces(
        &mut self,
        parent: &Trace,
        feature_block: &FeatureLazyBlock,
    ) -> Vec<TraceId> {
        feature_block
            .stmts
            .iter()
            .map(|stmt| self.feature_stmt_traces(parent, stmt.clone()))
            .flatten()
            .collect()
    }

    pub fn feature_func_block_subtraces(
        &mut self,
        parent: &Trace,
        feature_block: &FeatureFuncBlock,
    ) -> Option<Vec<TraceId>> {
        let instruction_sheet: &InstructionSheet = &feature_block.instruction_sheet;
        let mut arguments = vec![];
        let sample_id = self.restriction.opt_sample_id()?;
        if let Some(ref this_repr) = feature_block.opt_this {
            arguments.push(
                self.runtime()
                    .eval_feature_repr(this_repr, sample_id)
                    .unwrap(),
            )
        }
        let sample_id = self.restriction.opt_sample_id().unwrap();
        let eval_time = self.runtime();
        let evaluator: FeatureEvaluator<'_, 'static> = eval_time.evaluator(sample_id);
        let stack = arguments.into_iter().into();
        let history = exec_debug(
            self.runtime(),
            Some(&evaluator),
            instruction_sheet,
            stack,
            self.vm_config(),
        );
        let mut subtraces = vec![];
        subtraces.extend(self.func_stmts_traces(parent.id(), 4, &feature_block.stmts, &history));
        Some(subtraces)
    }

    pub fn feature_proc_block_subtraces(
        &mut self,
        parent: &Trace,
        feature_block: &FeatureProcBlock,
    ) -> Option<Vec<TraceId>> {
        let instruction_sheet: &InstructionSheet = &feature_block.instruction_sheet;
        let mut arguments = vec![];
        let sample_id = self.restriction.opt_sample_id()?;
        if let Some(ref this_repr) = feature_block.opt_this {
            arguments.push(
                self.runtime()
                    .eval_feature_repr(this_repr, sample_id)
                    .unwrap(),
            )
        }
        let sample_id = self.restriction.opt_sample_id().unwrap();
        let eval_time = self.runtime();
        let evaluator: FeatureEvaluator<'_, 'static> = eval_time.evaluator(sample_id);
        let stack = arguments.into_iter().into();
        let history = exec_debug(
            self.runtime(),
            Some(&evaluator),
            instruction_sheet,
            stack,
            self.vm_config(),
        );
        let mut subtraces = vec![];
        subtraces.extend(self.proc_stmts_traces(parent.id(), 4, &feature_block.stmts, &history));
        Some(subtraces)
    }
}
