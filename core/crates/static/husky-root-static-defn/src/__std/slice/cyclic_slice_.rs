mod end;
mod start;

use ::cyclic_slice::CyclicSlice;
use end::*;
use husky_visual_syntax::StaticVisualTy;
use start::*;
use std::any::TypeId;
use vm::*;

use super::*;
pub static STD_SLICE_CYCLIC_SLICE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "CyclicSlice",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "std::slice::CyclicSlice",
        spatial_parameters: &[StaticSpatialParameter {
            name: "E",
            variant: StaticGenericPlaceholderVariant::Type { traits: &[] },
        }],
        ty_members: &[
            &STD_SLICE_CYCLIC_SLICE_START_DEFN,
            &STD_SLICE_CYCLIC_SLICE_END_DEFN,
            &STD_SLICE_CYCLIC_SLICE_FIRST_DEFN,
            &STD_SLICE_CYCLIC_SLICE_LAST_DEFN,
        ],
        static_trait_impls: &[],
        variants: &[],
        kind: TyKind::Struct,
        visual_ty: StaticVisualTy::Group,
        opt_type_call: None,
    },
    dev_src: husky_dev_utils::__static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_FIRST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "firstx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
            copy_fp: linkage_fp!(generic_cyclic_slice_first_copy),
            eval_ref_fp: linkage_fp!(generic_cyclic_slice_first_eval_ref),
            temp_ref_fp: linkage_fp!(generic_cyclic_slice_first_temp_ref),
            temp_mut_fp: linkage_fp!(generic_cyclic_slice_first_temp_mut),
            move_fp: linkage_fp!(generic_cyclic_slice_first_move),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: __static_dev_src!(),
};

fn generic_cyclic_slice_first_copy<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}

unsafe fn generic_cyclic_slice_first_eval_ref<'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let generic_cyclic_slice: &'eval GenericCyclicSlice<'eval> = values[0].downcast_eval_ref();
    generic_cyclic_slice.first().unwrap().bind_eval_ref()
}

unsafe fn generic_cyclic_slice_first_temp_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let generic_cyclic_slice: &GenericCyclicSlice<'eval> = values[0].downcast_temp_ref();
    generic_cyclic_slice.first().unwrap().bind_temp_ref()
}

fn generic_cyclic_slice_first_temp_mut<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!("deprecated")
}

fn generic_cyclic_slice_first_move<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}

pub static STD_SLICE_CYCLIC_SLICE_LAST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "lastx",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(__Linkage::Member(&__MemberLinkage {
            copy_fp: linkage_fp!(generic_cyclic_slice_last_copy),
            eval_ref_fp: linkage_fp!(generic_cyclic_slice_last_eval_ref),
            temp_ref_fp: linkage_fp!(generic_cyclic_slice_last_temp_ref),
            temp_mut_fp: linkage_fp!(generic_cyclic_slice_last_temp_mut),
            move_fp: linkage_fp!(generic_cyclic_slice_last_move),
        })),
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: __static_dev_src!(),
};

fn generic_cyclic_slice_last_copy<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}

unsafe fn generic_cyclic_slice_last_eval_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let generic_cyclic_slice: &'eval GenericCyclicSlice<'eval> = values[0].downcast_eval_ref();
    generic_cyclic_slice.last().unwrap().bind_eval_ref()
}

unsafe fn generic_cyclic_slice_last_temp_ref<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    let generic_cyclic_slice: &GenericCyclicSlice<'eval> = values[0].downcast_temp_ref();
    generic_cyclic_slice.last().unwrap().bind_temp_ref()
}

fn generic_cyclic_slice_last_temp_mut<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!("deprecated")
}

fn generic_cyclic_slice_last_move<'temp, 'eval>(
    opt_ctx: Option<&dyn __EvalContext<'eval>>,
    values: &mut [__Register<'eval>],
) -> __Register<'eval> {
    todo!()
}