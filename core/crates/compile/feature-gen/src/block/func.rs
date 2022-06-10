use super::*;
use avec::Avec;
use vm::InstructionSheet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FeatureFuncBlock {
    pub opt_this: Option<FeatureRepr>,
    pub feature: FeaturePtr,
    pub file: FilePtr,
    pub range: TextRange,
    pub eval_id: FeatureEvalId,
    pub stmts: Avec<FuncStmt>,
    pub instruction_sheet: Arc<InstructionSheet>,
}

impl std::hash::Hash for FeatureFuncBlock {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}