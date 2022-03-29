use crate::*;
use vm::*;

use super::FeatureEvaluator;

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    pub(super) fn eval_feature_repr(&mut self, repr: &FeatureRepr) -> EvalResult<'eval> {
        match repr {
            FeatureRepr::Expr(expr) => self.eval_feature_expr(expr),
            FeatureRepr::Block(block) => self.eval_feature_block(block),
        }
    }
}