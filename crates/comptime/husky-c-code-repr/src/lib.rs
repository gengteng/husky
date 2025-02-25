use std::fmt::Display;

use convert_case::{Case, Casing};
pub struct CPrimitiveTypeRegistrationHeader<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CPrimitiveTypeRegistrationHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let uppercase_ty = ty.to_uppercase();
        write!(
            f,
            r#"
// {ty}
extern bool __{ty}_primitive_value_to_bool(__RegisterData data);
extern void *__{ty}_primitive_value_to_box(__RegisterData data);
extern void *__{ty}_clone(void const*);
extern void __{ty}_drop(void const*);
extern bool __{ty}_eq(void const*, void const*);
extern void __{ty}_assign(__Register *);
extern const __RegisterTyVTable __{uppercase_ty}_VTABLE;
        "#
        )
    }
}

pub struct CPrimitiveTypeRegistrationSource<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CPrimitiveTypeRegistrationSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let uppercase_ty = ty.to_uppercase();
        write!(
            f,
            r#"
const __RegisterTyVTable __{uppercase_ty}_VTABLE = {{
    .typename_str = "{ty}",
    .primitive_value_to_bool = __{ty}_primitive_value_to_bool,
    .primitive_value_to_box = __{ty}_primitive_value_to_box,
    .clone = __{ty}_clone,
    .drop = __{ty}_drop,
    .eq = __{ty}_eq,
    .assign = __{ty}_assign,
}};
"#
        )
    }
}

pub struct CNonPrimitiveTypeRegistrationHeader<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CNonPrimitiveTypeRegistrationHeader<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let lower_snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            f,
            r#"
// {ty}
extern void *__{lower_snake_ty}_clone(void const*);
extern void __{lower_snake_ty}_drop(void const*);
extern bool __{lower_snake_ty}_eq(void const*, void const*);
extern void __{lower_snake_ty}_assign(__Register*);
extern const __RegisterTyVTable __{upper_snake_ty}_VTABLE;
        "#
        )
    }
}

pub struct CNonPrimitiveTypeRegistrationSource<'a> {
    pub ty: &'a str,
}

impl<'a> Display for CNonPrimitiveTypeRegistrationSource<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ty = self.ty;
        let lower_snake_ty = ty.to_case(Case::Snake);
        let upper_snake_ty = ty.to_case(Case::UpperSnake);
        write!(
            f,
            r#"
const __RegisterTyVTable __{upper_snake_ty}_VTABLE = {{
    .typename_str = "{ty}",
    .primitive_value_to_bool = 0,
    .primitive_value_to_box = 0,
    .clone = __{lower_snake_ty}_clone,
    .drop = __{lower_snake_ty}_drop,
    .eq = __{lower_snake_ty}_eq,
    .assign = __{lower_snake_ty}_assign,
}};
"#
        )
    }
}
