// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;

use crate::{*, naive::*};
use vm::*;


// NaiveI32Internal
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_clone(data: *mut ()) -> *mut () {
    Box::<NaiveI32Internal>::into_raw(Box::new((*(data as *mut NaiveI32Internal)).clone())) as *mut ()
}
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_drop(data: *mut ()) {
    Box::from_raw(data as *mut NaiveI32Internal);
}
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_eq(this: &(), other: &()) -> bool {
    *(this as *const () as *const NaiveI32Internal) == *(other as *const () as *const NaiveI32Internal)
}
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<NaiveI32Internal>(&__NAIVE_I_32_INTERNAL_VTABLE) = registers[1].downcast_move(&__NAIVE_I_32_INTERNAL_VTABLE)
}
#[no_mangle]
pub static __NAIVE_I_32_INTERNAL_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_value_to_box: None,
    clone: __naive_i_32_internal_clone,
    drop: __naive_i_32_internal_drop,
    eq: __naive_i_32_internal_eq,
    assign: __naive_i_32_internal_assign,
    typename_str_hash_u64: 7406184017455031898,
    typename_str: "NaiveI32Internal",
};