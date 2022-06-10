use vm::{EvalResult, EvalValue, VMRuntimeError, VMRuntimeResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_stmt(&mut self, stmt: &FeatureStmt) -> EvalResult<'eval> {
        match stmt.variant {
            FeatureStmtVariant::Init { .. } => Ok(EvalValue::Undefined),
            FeatureStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(EvalValue::Undefined)
                } else {
                    Err(VMRuntimeError {
                        message: format!("assertion failed"),
                    })
                }
            }
            FeatureStmtVariant::Return { ref result } => self.eval_feature_expr(result),
            FeatureStmtVariant::ConditionFlow { ref branches, .. } => {
                for branch in branches {
                    let execute_branch: bool = match branch.variant {
                        FeatureBranchVariant::If { ref condition } => self.satisfies(condition)?,
                        FeatureBranchVariant::Elif { ref condition } => {
                            self.satisfies(condition)?
                        }
                        FeatureBranchVariant::Else => true,
                    };
                    if execute_branch {
                        return self.eval_feature_lazy_block(&branch.block);
                    }
                }
                Ok(EvalValue::Undefined)
            }
        }
    }

    fn satisfies(&mut self, condition: &FeatureExpr) -> VMRuntimeResult<bool> {
        Ok(self.eval_feature_expr(condition)?.primitive().to_bool())
    }
}