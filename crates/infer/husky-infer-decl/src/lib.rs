mod call_form;
mod feature;
mod impl_parse;
mod input;
mod member;
mod target;
mod trai;
mod ty;

pub use call_form::*;
use husky_print_utils::p;
pub use input::*;
pub use member::*;
pub use trai::*;
pub use ty::*;

use feature::*;
use fold::FoldableStorage;
use husky_ast::*;
use husky_defn_head::*;
use husky_entity_kind::TyKind;
use husky_entity_route::*;
use husky_entity_syntax::*;
use husky_infer_error::*;
use husky_instantiate::*;
use husky_liason_semantics::*;
use husky_opn_syntax::*;
use husky_static_defn::*;
use husky_word::{CustomIdentifier, RootIdentifier};
use std::sync::Arc;
use target::*;

#[salsa::query_group(DeclQueryGroupStorage)]
pub trait DeclQueryGroup: EntitySyntaxQueryGroup + husky_ast::AstQueryGroup {
    fn entity_call_form_decl(
        &self,
        call_route: EntityRoutePtr,
    ) -> InferQueryResultArc<CallFormDecl>;
    fn value_call_form_decl(&self, ty: EntityRoutePtr) -> InferQueryResultArc<CallFormDecl>;
    fn ty_decl(&self, ty: EntityRoutePtr) -> InferQueryResultArc<TyDecl>;
    fn trait_decl(&self, trai: EntityRoutePtr) -> InferResultArc<TraitDecl>;
    fn feature_decl(&self, feature_entity: EntityRoutePtr) -> InferResultArc<FeatureDecl>;
    fn target_input_ty(&self) -> InferResult<EntityRoutePtr>;
    fn target_output_ty(&self) -> InferResult<EntityRoutePtr>;
    fn implement_target(&self, ty: EntityRoutePtr) -> InferResult<EntityRoutePtr>;
    fn member_idx(&self, member_route: EntityRoutePtr) -> MemberIdx;
    fn is_copyable(&self, ty: EntityRoutePtr) -> InferResult<bool>;
    fn is_clonable(&self, ty: EntityRoutePtr) -> InferResult<bool>;
}

pub(crate) fn is_copyable(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferResult<bool> {
    match ty.variant {
        EntityRouteVariant::Root { ident } => Ok(match ident {
            RootIdentifier::Void
            | RootIdentifier::I32
            | RootIdentifier::I64
            | RootIdentifier::F32
            | RootIdentifier::F64
            | RootIdentifier::B32
            | RootIdentifier::B64
            | RootIdentifier::Bool
            | RootIdentifier::Ref
            | RootIdentifier::ThickFp => true,
            RootIdentifier::Vec => false,
            RootIdentifier::Tuple => false,
            RootIdentifier::Array => false,
            RootIdentifier::DatasetType => false,
            RootIdentifier::TypeType => false,
            RootIdentifier::TraitType => false,
            RootIdentifier::ModuleType => false,
            RootIdentifier::RefMut => false,
            RootIdentifier::Option => db.is_copyable(ty.entity_route_argument(0))?,
            _ => {
                p!(ident);
                panic!()
            }
        }),
        _ => {
            let ty_decl = db.ty_decl(ty)?;
            Ok(ty_decl
                .trait_impl(db.entity_route_menu().copy_trait)
                .is_some())
        }
    }
}

pub(crate) fn is_clonable(db: &dyn DeclQueryGroup, ty: EntityRoutePtr) -> InferResult<bool> {
    let ty_decl = db.ty_decl(ty)?;
    Ok(ty_decl
        .trait_impl(db.entity_route_menu().clone_trait)
        .is_some())
}
