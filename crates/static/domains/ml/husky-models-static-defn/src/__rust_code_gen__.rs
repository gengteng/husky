// this is generated by husky_vm_interface_code_gen::rust_code::write_rust_code
// do not modify by hand

use crate::*;

type void = ();
type b32 = u32;
type b64 = u64;

use crate::{*, naive::*};
use husky_vm::*;


// NaiveI32Internal
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    Box::<NaiveI32Internal>::into_raw(Box::new((*(data as *mut NaiveI32Internal)).clone())) as *mut std::ffi::c_void
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_drop(data: *mut std::ffi::c_void) {
    drop(Box::from_raw(data as *mut NaiveI32Internal))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {
    *(this as *const std::ffi::c_void as *const NaiveI32Internal) == *(other as *const std::ffi::c_void as *const NaiveI32Internal)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __naive_i_32_internal_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<NaiveI32Internal>(&__NAIVE_I_32_INTERNAL_VTABLE) = registers[1].downcast_move(&__NAIVE_I_32_INTERNAL_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __NAIVE_I_32_INTERNAL_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __naive_i_32_internal_clone,
    drop: __naive_i_32_internal_drop,
    eq: __naive_i_32_internal_eq,
    assign: __naive_i_32_internal_assign,
    typename_str_hash_u64: 7406184017455031898,
    typename_str: "NaiveI32Internal",
};

// NormalizeVmaxF32Internal
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __normalize_vmax_f_32_internal_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    Box::<NormalizeVmaxF32Internal>::into_raw(Box::new((*(data as *mut NormalizeVmaxF32Internal)).clone())) as *mut std::ffi::c_void
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __normalize_vmax_f_32_internal_drop(data: *mut std::ffi::c_void) {
    drop(Box::from_raw(data as *mut NormalizeVmaxF32Internal))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __normalize_vmax_f_32_internal_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {
    *(this as *const std::ffi::c_void as *const NormalizeVmaxF32Internal) == *(other as *const std::ffi::c_void as *const NormalizeVmaxF32Internal)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __normalize_vmax_f_32_internal_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<NormalizeVmaxF32Internal>(&__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE) = registers[1].downcast_move(&__NORMALIZE_VMAX_F_32_INTERNAL_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __NORMALIZE_VMAX_F_32_INTERNAL_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __normalize_vmax_f_32_internal_clone,
    drop: __normalize_vmax_f_32_internal_drop,
    eq: __normalize_vmax_f_32_internal_eq,
    assign: __normalize_vmax_f_32_internal_assign,
    typename_str_hash_u64: 12813832642459132925,
    typename_str: "NormalizeVmaxF32Internal",
};

// BoostingWithVmaxNormalizedInternal
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __boosting_with_vmax_normalized_internal_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    Box::<BoostingWithVmaxNormalizedInternal>::into_raw(Box::new((*(data as *mut BoostingWithVmaxNormalizedInternal)).clone())) as *mut std::ffi::c_void
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __boosting_with_vmax_normalized_internal_drop(data: *mut std::ffi::c_void) {
    drop(Box::from_raw(data as *mut BoostingWithVmaxNormalizedInternal))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __boosting_with_vmax_normalized_internal_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {
    *(this as *const std::ffi::c_void as *const BoostingWithVmaxNormalizedInternal) == *(other as *const std::ffi::c_void as *const BoostingWithVmaxNormalizedInternal)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __boosting_with_vmax_normalized_internal_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<BoostingWithVmaxNormalizedInternal>(&__BOOSTING_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE) = registers[1].downcast_move(&__BOOSTING_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __BOOSTING_WITH_VMAX_NORMALIZED_INTERNAL_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __boosting_with_vmax_normalized_internal_clone,
    drop: __boosting_with_vmax_normalized_internal_drop,
    eq: __boosting_with_vmax_normalized_internal_eq,
    assign: __boosting_with_vmax_normalized_internal_assign,
    typename_str_hash_u64: 7649255471547882478,
    typename_str: "BoostingWithVmaxNormalizedInternal",
};

// NarrowDownInternal
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __narrow_down_internal_clone(data: *mut std::ffi::c_void) -> *mut std::ffi::c_void {
    Box::<NarrowDownInternal>::into_raw(Box::new((*(data as *mut NarrowDownInternal)).clone())) as *mut std::ffi::c_void
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __narrow_down_internal_drop(data: *mut std::ffi::c_void) {
    drop(Box::from_raw(data as *mut NarrowDownInternal))
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __narrow_down_internal_eq(this: &std::ffi::c_void, other: &std::ffi::c_void) -> bool {
    *(this as *const std::ffi::c_void as *const NarrowDownInternal) == *(other as *const std::ffi::c_void as *const NarrowDownInternal)
}
#[rustfmt::skip]
#[no_mangle]
pub unsafe extern "C" fn __narrow_down_internal_assign(registers: *mut __Register) {
    let registers = std::slice::from_raw_parts_mut(registers, 2);
    *registers[0].downcast_temp_mut::<NarrowDownInternal>(&__NARROW_DOWN_INTERNAL_VTABLE) = registers[1].downcast_move(&__NARROW_DOWN_INTERNAL_VTABLE)
}
#[rustfmt::skip]
#[no_mangle]
pub static __NARROW_DOWN_INTERNAL_VTABLE: __RegisterTyVTable = __RegisterTyVTable {
    primitive_value_to_bool: None,
    primitive_ref_to_bool: None,
    primitive_value_to_box: None,
    clone: __narrow_down_internal_clone,
    drop: __narrow_down_internal_drop,
    eq: __narrow_down_internal_eq,
    assign: __narrow_down_internal_assign,
    typename_str_hash_u64: 6675027034438848780,
    typename_str: "NarrowDownInternal",
};
