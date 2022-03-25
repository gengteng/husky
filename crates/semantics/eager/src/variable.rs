use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EagerVariable {
    pub ident: CustomIdentifier,
    pub ty: ScopePtr,
    pub qual: Qual,
}

impl EagerVariable {
    pub(crate) fn from_input(input_placeholder: &InputPlaceholder) -> Self {
        EagerVariable {
            ident: input_placeholder.ident,
            ty: input_placeholder.ranged_ty.scope,
            qual: Qual::from_input(input_placeholder.contract),
        }
    }
}