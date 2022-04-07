mod call;
mod feature;
mod global;
mod memb;
mod traits;
mod ty;

pub use call::*;
pub use memb::*;
pub use traits::*;
pub use ty::*;

use ast::*;
use entity_route::*;
use entity_route_query::*;
use entity_syntax::RawTyKind;
use feature::*;
use file::FilePtr;
use fold::FoldStorage;
use global::*;
use infer_error::*;
use instantiate::*;
use std::sync::Arc;
use vm::RoutineFp;
use word::CustomIdentifier;

#[salsa::query_group(DeclQueryGroupStorage)]
pub trait DeclQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn call_decl(&self, scope: EntityRoutePtr) -> InferResultArc<RoutineDecl>;
    fn ty_decl(&self, scope: EntityRoutePtr) -> InferResultArc<TyDecl>;
    fn trait_decl(&self, scope: EntityRoutePtr) -> InferResultArc<TraitDecl>;
    fn feature_decl(&self, scope: EntityRoutePtr) -> InferResultArc<FeatureSignature>;
    fn global_input_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn global_output_ty(&self, main_file: FilePtr) -> InferResult<EntityRoutePtr>;
    fn vec_decl(&self) -> Arc<TyDecl>;
}