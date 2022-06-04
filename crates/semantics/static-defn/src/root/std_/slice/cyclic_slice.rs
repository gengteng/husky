mod end;
mod start;

use end::*;
use start::*;
use std::any::TypeId;
use vm::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CyclicSlice<'a, T> {
    pub start: i32,
    pub end: i32,
    pub total: &'a [T],
}

impl<'a, T> CyclicSlice<'a, T> {
    pub fn first(&self) -> Option<&T> {
        if self.total.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(&self.total[self.start.rem_euclid(self.total.len() as i32) as usize])
        }
    }
    pub fn last(&self) -> Option<&T> {
        if self.total.len() == 0 {
            None
        } else if self.start >= self.end {
            None
        } else {
            Some(&self.total[(self.end - 1).rem_euclid(self.total.len() as i32) as usize])
        }
    }
    pub fn first_mut(&mut self) -> Option<&mut T> {
        todo!()
    }
    pub fn last_mut(&mut self) -> Option<&mut T> {
        todo!()
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValue<'eval> for CyclicSlice<'eval, T> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::CyclicSlice(Box::new(T::static_type_id()))
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "CyclicSlice".into()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }
}

pub static STD_SLICE_CYCLIC_SLICE_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "CyclicSlice",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Ty {
        base_route: "std::slice::CyclicSlice",
        generic_parameters: &[StaticGenericPlaceholder {
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
        visualizer: StaticVisualizer::Vec,
        opt_type_call: None,
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub static STD_SLICE_CYCLIC_SLICE_FIRST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "first",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::MemberAccess {
                copy_access: linkage!(generic_cyclic_slice_first_copy, 1),
                eval_ref_access: linkage!(generic_cyclic_slice_first_eval_ref, 1),
                temp_ref_access: linkage!(generic_cyclic_slice_first_temp_ref, 1),
                temp_mut_access: linkage!(generic_cyclic_slice_first_mut, 1),
                move_access: linkage!(generic_cyclic_slice_first_move, 1),
            },
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_cyclic_slice_first_copy<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    todo!()
}

fn generic_cyclic_slice_first_eval_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> = values[0].downcast_ref();
    match generic_cyclic_slice.first() {
        Some(value) => Ok(value.bind_eval_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_first_temp_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> = values[0].downcast_ref();
    match generic_cyclic_slice.first() {
        Some(value) => Ok(value.bind_temp_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_first_mut<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let (generic_cyclic_slice, stack_idx, gen): (
        &mut CyclicSlice<'eval, MemberValue<'eval>>,
        _,
        _,
    ) = values[0].downcast_mut_full();
    match generic_cyclic_slice.first_mut() {
        Some(value) => Ok(value.binding_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_first_move<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    todo!()
}

pub static STD_SLICE_CYCLIC_SLICE_LAST_DEFN: EntityStaticDefn = EntityStaticDefn {
    name: "last",
    subscopes: &[],
    variant: EntityStaticDefnVariant::Method {
        this_liason: ParameterLiason::MemberAccess,
        parameters: &[],
        output_ty: "E",
        generic_parameters: &[],
        kind: MethodStaticDefnVariant::TypeMethod {
            source: LinkageSource::MemberAccess {
                copy_access: linkage!(generic_cyclic_slice_last_copy, 1),
                eval_ref_access: linkage!(generic_cyclic_slice_last_eval_ref, 1),
                temp_ref_access: linkage!(generic_cyclic_slice_last_temp_ref, 1),
                temp_mut_access: linkage!(generic_cyclic_slice_last_mut, 1),
                move_access: linkage!(generic_cyclic_slice_last_move, 1),
            },
        },
        output_liason: OutputLiason::MemberAccess {
            member_liason: MemberLiason::Mutable,
        },
    },
    dev_src: static_dev_src!(),
};

fn generic_cyclic_slice_last_copy<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    todo!()
}

fn generic_cyclic_slice_last_eval_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> = values[0].downcast_ref();
    match generic_cyclic_slice.last() {
        Some(value) => Ok(value.bind_eval_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_last_temp_ref<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let generic_cyclic_slice: &CyclicSlice<'eval, MemberValue<'eval>> = values[0].downcast_ref();
    match generic_cyclic_slice.last() {
        Some(value) => Ok(value.bind_temp_ref()),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_last_mut<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    let (generic_cyclic_slice, stack_idx, gen): (
        &mut CyclicSlice<'eval, MemberValue<'eval>>,
        _,
        _,
    ) = values[0].downcast_mut_full();
    match generic_cyclic_slice.last_mut() {
        Some(value) => Ok(value.binding_mut(stack_idx)),
        None => Err(vm_runtime_error!("empty vec")),
    }
}

fn generic_cyclic_slice_last_move<'temp, 'eval>(
    values: &mut [TempValue<'temp, 'eval>],
) -> VMRuntimeResult<TempValue<'temp, 'eval>> {
    todo!()
}