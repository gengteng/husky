use crate::*;
use __husky::init::*;

#[no_mangle]
pub extern "C" fn get_linkages() -> &'static [(__StaticLinkageKey, __Linkage)] {
    LINKAGES
}

pub static LINKAGES: &[(__StaticLinkageKey, __Linkage)] = &[
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example2::f1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    __Register::new_box::<A>(f1(), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            some base f1 as fn() -> A
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example2::A::get_x",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &A = __arguments[0].downcast_temp_ref(&__registration__::__A_VTABLE);
                    __this.get_x().to_register()
                }
                __wrapper
            },
            some base A::get_x as fn(&'static A) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example2::A::get_x_plus_constant",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &A = __arguments[0].downcast_temp_ref(&__registration__::__A_VTABLE);
                    __this.get_x_plus_constant().to_register()
                }
                __wrapper
            },
            some base A::get_x_plus_constant as fn(&'static A) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example2::A::get_x_squared",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let __this: &A = __arguments[0].downcast_temp_ref(&__registration__::__A_VTABLE);
                    __this.get_x_squared().to_register()
                }
                __wrapper
            },
            some base A::get_x_squared as fn(&'static A) -> i32
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example2::g1",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    g1().to_register()
                }
                __wrapper
            },
            some base g1 as fn() -> i32
        ),
    ),
    (
        __StaticLinkageKey::TypeCall {
            ty: "test_struct_example2::A",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    let x: i32 = __arguments[0].downcast_i32();
                    let y: i32 = todo!();
                    __Register::new_box::<A>(A::__call__(x, y), &__registration__::__A_VTABLE)
                }
                __wrapper
            },
            some base A::__call__ as fn(i32, i32) -> A
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_struct_example2::A",
            field_ident: "x",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            A,
            __registration__::__A_VTABLE,
            i32,
            __registration__::__I32_VTABLE,
            x
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_struct_example2::A",
            field_ident: "y",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            A,
            __registration__::__A_VTABLE,
            i32,
            __registration__::__I32_VTABLE,
            y
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_struct_example2::A",
            field_ident: "z",
        },
        eager_field_linkage!(
            immutable,
            Intrinsic,
            Direct,
            A,
            __registration__::__A_VTABLE,
            i32,
            __registration__::__I32_VTABLE,
            z
        ),
    ),
    (
        __StaticLinkageKey::StructField {
            this_ty: "test_struct_example2::A",
            field_ident: "w",
        },
        lazy_field_linkage!(
            A,
            __registration__::__A_VTABLE,
            i32,
            __registration__::__I32_VTABLE,
            w
        ),
    ),
    (
        __StaticLinkageKey::Routine {
            route: "test_struct_example2::f3",
        },
        transfer_linkage!(
            {
                unsafe fn __wrapper<'eval>(
                    __arguments: &mut [__Register<'eval>],
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                ) -> __Register<'eval> {
                    f3().to_register()
                }
                __wrapper
            },
            some base f3 as fn() -> ()
        ),
    ),
];
