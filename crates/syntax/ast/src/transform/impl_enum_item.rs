use token::{Token, TokenKind};

use crate::*;

use super::utils::identify;

impl<'a> AstTransformer<'a> {
    pub(super) fn parse_enum_variant(&mut self, tokens: &[Token]) -> AstResult<AstKind> {
        Ok(if tokens.len() == 1 {
            AstKind::EnumVariant {
                ident: identify!(Some(self.file), tokens[0]),
                raw_variant_kind: RawEnumVariantKind::Constant,
            }
        } else {
            todo!()
        })
    }
}