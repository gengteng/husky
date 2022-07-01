use vm::{EvalError, EvalResult, EvalValue};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(crate) fn husky_feature_eval_stmt(
        &mut self,
        stmt: &FeatureStmt,
    ) -> EvalResult<EvalValue<'eval>> {
        match stmt.variant {
            FeatureLazyStmtVariant::Init { .. } => Ok(EvalValue::Undefined),
            FeatureLazyStmtVariant::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(EvalValue::Undefined)
                } else {
                    Err(EvalError::Normal {
                        message: format!("assertion failed"),
                    }
                    .into())
                }
            }
            FeatureLazyStmtVariant::Return { ref result } => self.husky_feature_eval_expr(result),
            FeatureLazyStmtVariant::ReturnXml { ref result } => {
                self.husky_feature_eval_xml_expr(result)
            }
            FeatureLazyStmtVariant::ConditionFlow { ref branches, .. } => {
                for branch in branches {
                    let execute_branch: bool = match branch.variant {
                        FeatureBranchVariant::If { ref condition } => self.satisfies(condition)?,
                        FeatureBranchVariant::Elif { ref condition } => {
                            self.satisfies(condition)?
                        }
                        FeatureBranchVariant::Else => true,
                    };
                    if execute_branch {
                        return self.husky_feature_eval_lazy_block(&branch.block);
                    }
                }
                Ok(EvalValue::Undefined)
            }
        }
    }

    fn satisfies(&mut self, condition: &FeatureExpr) -> EvalResult<bool> {
        Ok(self
            .husky_feature_eval_expr(condition)?
            .primitive()
            .to_bool())
    }
}