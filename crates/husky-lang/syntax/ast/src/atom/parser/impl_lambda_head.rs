use crate::*;

use super::*;

// inner ops
impl<'a> AtomLRParser<'a> {
    pub(crate) fn lambda_head(&mut self) -> AstResult<Vec<(CustomIdentifier, Option<ScopeId>)>> {
        Ok(comma_list!(self, lambda_parameter!, "|"))
    }

    pub(crate) fn lambda_parameter(&mut self) -> AstResult<(CustomIdentifier, Option<ScopeId>)> {
        let ident = get!(self, custom_ident);
        let ty = if next_matches!(self, ":") {
            Some(get!(self.ty?))
        } else {
            None
        };
        Ok((ident, ty))
    }
}