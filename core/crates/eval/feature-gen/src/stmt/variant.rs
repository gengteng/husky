use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureStmtVariant {
    Init {
        varname: CustomIdentifier,
        value: Arc<FeatureLazyExpr>,
    },
    Assert {
        condition: Arc<FeatureLazyExpr>,
    },
    Return {
        result: Arc<FeatureLazyExpr>,
    },
    ReturnXml {
        result: Arc<FeatureXmlExpr>,
    },
    ConditionFlow {
        branches: Vec<Arc<FeatureBranch>>,
    },
}

impl FeatureStmtVariant {
    pub(super) fn opt_feature(&self, feature_interner: &FeatureInterner) -> Option<FeaturePtr> {
        match self {
            FeatureStmtVariant::Init { .. } => None,
            FeatureStmtVariant::Assert { condition } => {
                Some(feature_interner.intern(Feature::Assert {
                    condition: condition.feature,
                }))
            }
            FeatureStmtVariant::Return { result } => Some(result.feature),
            FeatureStmtVariant::ReturnXml { result } => Some(result.feature),
            FeatureStmtVariant::ConditionFlow { branches } => Some(
                feature_interner.intern(Feature::Branches {
                    branches: branches
                        .iter()
                        .map(|branch| match branch.variant {
                            FeatureBranchVariant::If { ref condition } => BranchedFeature {
                                condition: Some(condition.feature),
                                block: branch.block.feature,
                            },
                            FeatureBranchVariant::Elif { ref condition } => BranchedFeature {
                                condition: Some(condition.feature),
                                block: branch.block.feature,
                            },
                            FeatureBranchVariant::Else => BranchedFeature {
                                condition: None,
                                block: branch.block.feature,
                            },
                        })
                        .collect(),
                }),
            ),
        }
    }
}