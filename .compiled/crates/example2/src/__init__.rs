
use crate::*;
use __husky_root::__init_utils::*;

pub static LINKAGES : &[(__StaticLinkageKey, __Linkage)]= &[

    (
        __StaticLinkageKey::TypeCall {
            ty: "example2::A"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let x: i32 = __arguments[0].downcast_copy();
                __TempValue::OwnedEval(__OwnedValue::new(
                    A::__call__(x)
                    ))
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "example2::A",
            field_ident: "x",
        },
        eager_field_linkage!(A, x)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "example2::A",
            field_ident: "y",
        },
        eager_field_linkage!(A, y)
    ),
    (
        __StaticLinkageKey::StructEagerField {
            this_ty: "example2::A",
            field_ident: "z",
        },
        eager_field_linkage!(A, z)
    ),
    (
        __StaticLinkageKey::FeatureEagerBlock {
            route: "example2::A::w",
        },
        lazy_field_linkage!(A, w)
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example2::f1"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::OwnedEval(__OwnedValue::new(
                    f1()
                    ))
            }
            __wrapper
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example2::f3"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    f3()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),
    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example2::A::get_x"
        },
        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                let __this: &A = __arguments[0].downcast_temp_ref();
                __TempValue::Copyable(
                    __this.get_x()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
    (
        __StaticLinkageKey::Routine {
            routine: "example2::g1"
        },

        specific_transfer_linkage!({
            fn __wrapper<'temp, 'eval>(
                __opt_ctx: Option<&__EvalContext<'eval>>,
                __arguments: &mut [__TempValue<'temp, 'eval>],
            ) -> __TempValue<'temp, 'eval> {
                __TempValue::Copyable(
                    g1()
                    .__take_copyable_dyn())
            }
            __wrapper
        }),

    ),
];