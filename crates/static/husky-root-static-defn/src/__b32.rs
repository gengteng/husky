use husky_static_visualizer::StaticVisualTy;

use super::*;

pub static B32_TYPE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "b32",
    items: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "b32",
        spatial_parameters: &[],
        trait_impls: &[],
        ty_members: &[
            &B32_LEADING_ZEROS,
            &B32_TRAILING_ZEROS,
            &B32_SPAN,
            &B32_LAST_BITS,
        ],
        variants: &[],
        kind: TyKind::Primitive,
        visualizer: StaticVisualizer {
            visual_ty: StaticVisualTy::B32,
            fp: StaticVisualizerFp(|_| todo!()),
        },
        opt_type_call: None,
    },
    dev_src: static_dev_src!(),
};

pub static B32_LEADING_ZEROS: EntityStaticDefn = EntityStaticDefn {
    name: "leading_zeros",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            |values, _| { (values[0].downcast_b32().leading_zeros() as i32).to_register() },
            some base (|x| x.leading_zeros() as i32) as fn(u32) -> i32
        )),
    },
    dev_src: static_dev_src!(),
};

pub static B32_TRAILING_ZEROS: EntityStaticDefn = EntityStaticDefn {
    name: "ctz",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            |values, _| values[0]. downcast_b32().ctz().to_register(),
            some base u32::ctz as fn(u32) -> i32
        )),
    },
    dev_src: static_dev_src!(),
};

pub static B32_SPAN: EntityStaticDefn = EntityStaticDefn {
    name: "span",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[],
        output_ty: "i32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            |values, _| values[0]. downcast_b32().span().to_register(),
            some base u32::span as fn(u32) -> i32
        )),
    },
    dev_src: static_dev_src!(),
};

pub static B32_LAST_BITS: EntityStaticDefn = EntityStaticDefn {
    name: "last_bits",
    items: &[],
    variant: EntityStaticDefnVariant::Method {
        this_modifier: ParameterModifier::None,
        parameters: &[StaticParameter {
            name: "k",
            modifier: ParameterModifier::None,
            ty: "i32",
        }],
        output_ty: "b32",
        output_liason: OutputModifier::Transfer,
        spatial_parameters: &[],
        method_static_defn_kind: MethodStaticDefnKind::TypeMethod,
        opt_linkage: Some(transfer_linkage!(
            |values, _| {
                let b = values[0].downcast_b32();
                let i = values[1].downcast_i32();
                let last_bits = b & ((1 << i) - 1);
                last_bits.to_register()
            },
            some base u32::last_bits as fn(u32, i32) -> u32
        )),
    },
    dev_src: static_dev_src!(),
};
