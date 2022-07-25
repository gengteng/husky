use crate::*;

impl __StaticInfo for i32 {
    type __StaticSelf = Self;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "i32".into()
    }
}

impl __Registrable for i32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self as u64)
    }
}

impl __StaticInfo for () {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "void".into()
    }
}

impl __Registrable for () {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(0)
    }
}

impl __StaticInfo for f32 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "f32".into()
    }
}

impl __Registrable for f32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self as u64)
    }
}

impl __StaticInfo for u32 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "u32".into()
    }
}

impl __Registrable for u32 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        todo!()
    }
}

impl __StaticInfo for u64 {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "u64".into()
    }
}

impl __Registrable for u64 {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self)
    }
}

impl __StaticInfo for bool {
    type __StaticSelf = Self;
    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "bool".into()
    }
}

impl __Registrable for bool {
    unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
        __Register::new_direct::<Self>(self as u64)
    }
}

// pub trait __AsU64: Copy {
//     fn __as_u64(self) -> u64 {
//         panic!()
//     }
// }

// impl __AsU64 for bool {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for i32 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for i64 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for u32 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for u64 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for f32 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }

// impl __AsU64 for f64 {
//     fn __as_u64(self) -> u64 {
//         self as u64
//     }
// }