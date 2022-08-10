use husky_eager_semantics::EagerOpnVariant;

use crate::*;

impl<'eval> Trace {
    pub fn subtraces_container_class(&self) -> Option<SubtracesContainerClass> {
        if !self.raw_data.can_have_subtraces {
            return None;
        }
        match self.variant {
            TraceVariant::Main(_)
            | TraceVariant::EntityFeature { .. }
            | TraceVariant::FeatureStmt(_)
            | TraceVariant::FeatureBranch(_)
            | TraceVariant::FeatureCallArgument { .. }
            | TraceVariant::FuncStmt { .. }
            | TraceVariant::ProcStmt { .. }
            | TraceVariant::LoopFrame { .. }
            | TraceVariant::CallHead { .. }
            | TraceVariant::FuncBranch { .. }
            | TraceVariant::ProcBranch { .. } => None,
            TraceVariant::Module { .. } => todo!(),
            TraceVariant::FeatureExpr(ref expr) => match expr.variant {
                FeatureExprVariant::RoutineCall { .. } => Some(SubtracesContainerClass::Call),
                FeatureExprVariant::EntityFeature { .. } => None,
                FeatureExprVariant::RecordDerivedField { .. }
                | FeatureExprVariant::StructDerivedLazyField { .. } => {
                    Some(SubtracesContainerClass::Call)
                }
                _ => None,
            },
            TraceVariant::EagerExpr { ref expr, .. } => match expr.variant {
                EagerExprVariant::Variable { .. } | EagerExprVariant::PrimitiveLiteral(_) => None,
                EagerExprVariant::Opn {
                    ref opn_variant,
                    ref opds,
                    ..
                } => match opn_variant {
                    EagerOpnVariant::Field { .. } | EagerOpnVariant::Index { .. } => None,
                    EagerOpnVariant::Binary { .. }
                    | EagerOpnVariant::Prefix { .. }
                    | EagerOpnVariant::Suffix { .. } => {
                        if opds[0].ty().is_builtin() {
                            None
                        } else {
                            Some(SubtracesContainerClass::Call)
                        }
                    }
                    EagerOpnVariant::RoutineCall { .. } | EagerOpnVariant::MethodCall { .. } => {
                        Some(SubtracesContainerClass::Call)
                    }
                    EagerOpnVariant::TypeCall { .. } => todo!(),
                    EagerOpnVariant::NewVecFromList => todo!(),
                    EagerOpnVariant::ValueCall => todo!(),
                },
                EagerExprVariant::Lambda(_, _) => todo!(),
                EagerExprVariant::Bracketed(_) => panic!(),
                EagerExprVariant::ThisValue { .. } => todo!(),
                EagerExprVariant::ThisField { .. } => todo!(),
                EagerExprVariant::EnumKindLiteral(_) => todo!(),
                EagerExprVariant::EntityFeature { .. } => todo!(),
                EagerExprVariant::EntityFp { route } => todo!(),
            },
            TraceVariant::EagerCallArgument {
                name: ident,
                ref argument,
                ref history,
            } => todo!(),
        }
    }
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
pub enum SubtracesContainerClass {
    Call,
}