#![feature(const_trait_impl)]
#![feature(const_convert)]
mod __b32;
mod __clone;
mod __eq;
mod __f32;
mod __i32;
pub mod __init_utils;
pub mod __main_utils;
pub mod __std;
mod __vec;
pub mod domains;
mod etc;

pub use __b32::*;
pub use __clone::*;
pub use __eq::*;
pub use __f32::*;
pub use __i32::*;
pub use __std::*;
pub use __vec::*;
pub use domains::*;
pub use etc::*;
pub use husky_entity_route::EntityRoutePtr as __EntityRoutePtr;
pub use husky_eval_time::__ty_route_from_static_binded;
pub use husky_feature_eval::{__root::*, entity_uid, feature_ptr};
pub use husky_feature_gen::FeaturePtr as __FeaturePtr;
pub use serde::Serialize as __Serialize;
pub use serde_json::value::Value as __JsonValue;
pub use static_defn::__StaticLinkageKey;
pub use std::sync::Arc as __Arc;
pub use vm::{
    __AnyValue, __AnyValueDyn, __EvalContext, __EvalRef, __EvalValue, __HasStaticTypeInfo,
    __OwnedValue, __TempValue,
};

use __init_utils::*;
use dev_utils::__StaticDevSource;
use dev_utils::{__static_dev_src, dev_src};
use entity_kind::{EntityKind, FieldKind, MemberKind, RoutineKind, TyKind};
use husky_liason_semantics::{MemberLiason, OutputLiason, ParameterLiason};
use husky_visual_syntax::StaticVisualTy;
use static_defn::StaticParameter;
use static_defn::*;
use vm::*;
use word::RootIdentifier;

pub fn __resolve_root_defn(ident: RootIdentifier) -> &'static EntityStaticDefn {
    match ident {
        RootIdentifier::Void => &VOID_TYPE_DEFN,
        RootIdentifier::I32 => &I32_TYPE_DEFN,
        RootIdentifier::F32 => &F32_TYPE_DEFN,
        RootIdentifier::B32 => &B32_TYPE_DEFN,
        RootIdentifier::B64 => &B64_TYPE_DEFN,
        RootIdentifier::Bool => &BOOL_TYPE_DEFN,
        RootIdentifier::True => todo!(),
        RootIdentifier::False => todo!(),
        RootIdentifier::Vec => &VEC_TYPE_DEFN,
        RootIdentifier::Tuple => todo!(),
        RootIdentifier::Debug => todo!(),
        RootIdentifier::Std => &STD_DEFN,
        RootIdentifier::Core => todo!(),
        RootIdentifier::Mor => todo!(),
        RootIdentifier::Fp => todo!(),
        RootIdentifier::Fn => todo!(),
        RootIdentifier::FnMut => todo!(),
        RootIdentifier::FnOnce => todo!(),
        RootIdentifier::Array => todo!(),
        RootIdentifier::Domains => &DOMAINS_DEFN,
        RootIdentifier::DatasetType => &husky_datasets_static_defn::DATASET_TYPE_DEFN,
        RootIdentifier::TypeType => &TYPE_TYPE_DEFN,
        RootIdentifier::TraitType => &TRAIT_TYPE_DEFN,
        RootIdentifier::CloneTrait => &CLONE_TRAIT_DEFN,
        RootIdentifier::CopyTrait => todo!(),
        RootIdentifier::PartialEqTrait => todo!(),
        RootIdentifier::EqTrait => todo!(),
        RootIdentifier::ModuleType => &MODULE_TYPE_DEFN,
        RootIdentifier::Ref => panic!(),
        RootIdentifier::Option => panic!(),
        RootIdentifier::VisualType => todo!(),
    }
    .into()
}

pub static VOID_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "void",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "void",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visual_ty: StaticVisualTy::Void,
        opt_type_call: None,
    },
    dev_src: __static_dev_src!(),
};

pub static B64_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b64",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "b64",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visual_ty: StaticVisualTy::B64,
        opt_type_call: None,
    },
    dev_src: __static_dev_src!(),
};

pub static BOOL_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "bool",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "bool",
        spatial_parameters: &[],
        static_trait_impls: &[],
        ty_members: &[],
        variants: &[],
        kind: TyKind::Primitive,
        visual_ty: StaticVisualTy::Bool,
        opt_type_call: None,
    },
    dev_src: __static_dev_src!(),
};
