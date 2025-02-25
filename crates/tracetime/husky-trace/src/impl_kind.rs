use crate::*;

impl<'eval> TraceVariant<'eval> {
    pub fn kind(&self) -> TraceKind {
        match self {
            TraceVariant::Main(_) => TraceKind::Main,
            TraceVariant::Module { .. } => TraceKind::Module,
            TraceVariant::EntityFeature { ref repr, .. } => match repr.is_lazy() {
                true => TraceKind::EntityFeatureLazy,
                false => TraceKind::EntityFeatureEager,
            },
            TraceVariant::FeatureStmt(_) => TraceKind::FeatureStmt,
            TraceVariant::FeatureBranch(_) => TraceKind::FeatureBranch,
            TraceVariant::FeatureExpr(expr) => match expr.variant {
                FeatureLazyExprVariant::StructDerivedLazyField { ref repr, .. } => {
                    match repr.is_lazy() {
                        true => TraceKind::FeatureExprLazy,
                        false => TraceKind::FeatureExprEager,
                    }
                }
                FeatureLazyExprVariant::RecordDerivedField { .. } => TraceKind::FeatureExprLazy,
                FeatureLazyExprVariant::ModelCall { .. } => TraceKind::FeatureExprLazy,
                FeatureLazyExprVariant::EntityFeature { ref repr, .. } => match repr.is_lazy() {
                    true => TraceKind::FeatureExprLazy,
                    false => TraceKind::FeatureExprEager,
                },
                FeatureLazyExprVariant::EvalInput => TraceKind::FeatureExprEager,
                FeatureLazyExprVariant::NewRecord { .. } => TraceKind::FeatureExprLazy,
                FeatureLazyExprVariant::NewVecFromList { .. } => TraceKind::FeatureExprEager,
                _ => TraceKind::FeatureExprEager,
            },
            TraceVariant::FeatureCallArgument { .. } => TraceKind::FeatureCallArgument,
            TraceVariant::FuncStmt { .. } => TraceKind::FuncStmt,
            TraceVariant::ProcStmt { .. } => TraceKind::ProcStmt,
            TraceVariant::ProcBranch { .. } => TraceKind::ProcBranch,
            TraceVariant::FuncBranch { .. } => TraceKind::FuncBranch,
            TraceVariant::LoopFrame { .. } => TraceKind::LoopFrame,
            TraceVariant::EagerExpr { .. } => TraceKind::EagerExpr,
            TraceVariant::CallHead { .. } => TraceKind::CallHead,
            TraceVariant::EagerCallArgument { .. } => TraceKind::EagerCallArgument,
        }
    }
}
