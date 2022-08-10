use husky_atom::AtomContext;
use husky_text::TextRange;
use vec_like::VecMapEntry;

use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDecl {
    pub liason: ParameterLiason,
    pub ty: EntityRoutePtr,
    pub ident: CustomIdentifier,
}

impl VecMapEntry<CustomIdentifier> for ParameterDecl {
    fn key(&self) -> CustomIdentifier {
        self.ident
    }
}

impl ParameterDecl {
    pub fn from_static(symbol_context: &mut dyn AtomContext, input: &StaticParameter) -> Self {
        // opt_this_ty,
        Self {
            ty: symbol_context.parse_entity_route(input.ty).unwrap(),
            liason: input.liason,
            ident: symbol_context.entity_syntax_db().custom_ident(input.name),
        }
    }

    pub fn from_field(db: &dyn DeclQueryGroup, field_decl: &FieldDecl) -> InferResult<Self> {
        Ok(ParameterDecl {
            liason: ParameterLiason::from_member(
                field_decl.liason,
                field_decl.ty,
                db.is_copyable(field_decl.ty)?,
            ),
            ty: field_decl.ty,
            ident: field_decl.ident,
        })
    }

    pub fn from_parameter(db: &dyn DeclQueryGroup, parameter: &Parameter) -> InferResult<Self> {
        Ok(ParameterDecl {
            liason: parameter.ranged_liason.liason,
            ty: db.implement_target(parameter.ranged_ty.route)?,
            ident: parameter.ranged_ident.ident,
        })
    }

    pub fn instantiate(&self, ctx: &InstantiationContext) -> Self {
        Self {
            ty: self.ty.instantiate(ctx).take_entity_route(),
            liason: self.liason,
            ident: self.ident,
        }
    }

    pub fn implement(&self, implementor: &ImplementationContext) -> Self {
        todo!()
    }
}