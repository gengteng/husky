use crate::*;

use std::sync::Arc;

impl HuskyTraceTime {
    pub fn feature_repr_subtraces(
        &mut self,
        parent: &Trace,
        feature_repr: &FeatureRepr,
    ) -> Vec<TraceId> {
        match feature_repr {
            FeatureRepr::Value { .. } => todo!(),
            FeatureRepr::Expr(_) => todo!(),
            FeatureRepr::LazyBlock(feature_block) => {
                self.feature_lazy_block_subtraces(parent, feature_block)
            }
            FeatureRepr::FuncBlock(feature_block) => {
                self.feature_func_block_subtraces(parent, feature_block)
            }
            FeatureRepr::ProcBlock(_) => todo!(),
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
    ) -> Vec<TraceId> {
        let instruction_sheet: &InstructionSheet = &feature_block.instruction_sheet;
        let mut arguments = vec![];
        let sample_id = self.attention.opt_sample_id().unwrap();
        if let Some(ref this_repr) = feature_block.opt_this {
            arguments.push(
                self.eval_time()
                    .eval_feature_repr(this_repr, sample_id)
                    .unwrap()
                    .into_stack()
                    .unwrap(),
            )
        }
        let history = exec_debug(
            self.eval_time().upcast(),
            instruction_sheet,
            arguments.into_iter(),
            self.eval_time().verbose(),
        );
        let mut subtraces = vec![];
        subtraces.extend(self.func_stmts_traces(parent.id(), 4, &feature_block.stmts, &history));
        subtraces
    }
}