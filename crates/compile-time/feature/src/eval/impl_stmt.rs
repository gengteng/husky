use vm::{EvalResult, EvalValue, VMError, VMResult};

use crate::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_stmt(&mut self, stmt: &FeatureStmt) -> EvalResult<'eval> {
        match stmt.kind {
            FeatureStmtKind::Init { .. } => Ok(EvalValue::Undefined),
            FeatureStmtKind::Assert { ref condition } => {
                if self.satisfies(condition)? {
                    Ok(EvalValue::Undefined)
                } else {
                    Err(VMError::AssertionFailure)
                }
            }
            FeatureStmtKind::Return { ref result } => self.eval_feature_expr(result),
            FeatureStmtKind::BranchGroup { ref branches, .. } => {
                for branch in branches {
                    let execute_branch: bool = match branch.kind {
                        FeatureBranchKind::If { ref condition } => self.satisfies(condition)?,
                        FeatureBranchKind::Elif { ref condition } => self.satisfies(condition)?,
                        FeatureBranchKind::Else => true,
                    };
                    if execute_branch {
                        return self.eval_feature_block(&branch.block);
                    }
                }
                Ok(EvalValue::Undefined)
            }
        }
    }

    fn satisfies(&mut self, condition: &FeatureExpr) -> VMResult<bool> {
        Ok(match self.eval_feature_expr(condition)? {
            EvalValue::Primitive(value) => match value {
                PrimitiveValue::I32(_) => todo!(),
                PrimitiveValue::F32(_) => todo!(),
                PrimitiveValue::B32(_) => todo!(),
                PrimitiveValue::B64(_) => todo!(),
                PrimitiveValue::Bool(b) => b,
                PrimitiveValue::Void => todo!(),
            },
            _ => todo!(),
        })
    }
}