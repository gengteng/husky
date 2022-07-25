use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct __MemberLinkage {
    pub copy_fp: __LinkageFp,
    pub eval_ref_fp: __LinkageFp,
    pub temp_ref_fp: __LinkageFp,
    pub temp_mut_fp: __LinkageFp,
    pub move_fp: __LinkageFp,
}

#[cfg(feature = "binding")]
use husky_vm_binding::Binding;

#[cfg(feature = "binding")]
impl __MemberLinkage {
    pub fn bind(&self, binding: Binding) -> __LinkageFp {
        match binding {
            Binding::EvalRef => self.eval_ref_fp,
            Binding::TempRef => self.temp_ref_fp,
            Binding::TempRefMut => self.temp_mut_fp,
            Binding::Move => self.move_fp,
            Binding::Copy => self.copy_fp,
        }
    }
}

#[macro_export]
macro_rules! method_elem_linkage {
    ($Type: ty, $method_name: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_fp: method_elem_copy_fp!($Type, $method_name),
            eval_ref_fp: method_elem_eval_ref_fp!($Type, $method_name),
            temp_ref_fp: method_elem_temp_ref_fp!($Type, $method_name),
            temp_mut_fp: method_elem_temp_mut_fp!($Type, $method_name),
            move_fp: method_elem_move_fp!($Type, $method_name),
        })
    }};
}

#[macro_export]
macro_rules! eager_field_linkage {
    ($Type: ty, $FieldTy: ty, $field: ident, $copy_kind: tt) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_fp: field_copy_fp!($Type, $FieldTy, $field, $copy_kind),
            eval_ref_fp: field_eval_ref_fp!($Type, $field),
            temp_ref_fp: field_temp_ref_fp!($Type, $field),
            temp_mut_fp: field_temp_mut_invalid_fp!($Type, $field),
            move_fp: field_move_fp!($Type, $field),
        })
    }};
}

#[macro_export]
macro_rules! lazy_field_linkage {
    ($Type: ty, $field: ident) => {{
        fn __wrapper<'eval>(
            __opt_ctx: Option<&dyn __EvalContext<'eval>>,
            values: &mut [__Register<'eval>],
        ) -> __Register {
            let this_value: &'eval $Type = values[0].downcast_eval_ref();
            __Register::new_eval_ref(this_value.$field(__opt_ctx.unwrap()))
        }
        transfer_linkage!(__wrapper, none)
    }};
}

#[macro_export]
macro_rules! eager_mut_field_linkage {
    ($Type: ty, $field: ident) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_fp: field_copy_fp!($Type, $field),
            eval_ref_fp: field_eval_ref_fp!($Type, $field),
            temp_ref_fp: field_temp_ref_fp!($Type, $field),
            temp_mut_fp: field_temp_mut_fp!($Type, $field),
            move_fp: field_move_fp!($Type, $field),
        })
    }};
}

#[macro_export]
macro_rules! index_linkage {
    ($Type: ty, $ElementType: ty, $copy_kind: tt) => {{
        __Linkage::Member(&__MemberLinkage {
            copy_fp: index_copy_fp!($Type, $ElementType, $copy_kind),
            eval_ref_fp: index_eval_ref_fp!($Type),
            temp_ref_fp: index_temp_ref_fp!($Type),
            temp_mut_fp: index_temp_mut_fp!($Type),
            move_fp: index_move_fp!($Type),
        })
    }};
}