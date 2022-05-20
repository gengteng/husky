use print_utils::p;

use crate::*;
use std::{any::TypeId, borrow::Cow, fmt::Debug, panic::RefUnwindSafe, sync::Arc};

#[derive(Debug, PartialEq, Eq)]
pub enum StaticTypeId {
    RustBuiltin(TypeId),
    HuskyBuiltin(HuskyBuiltinStaticTypeId),
    VecOf(Box<StaticTypeId>),
}

impl From<TypeId> for StaticTypeId {
    fn from(id: TypeId) -> Self {
        Self::RustBuiltin(id)
    }
}

impl From<HuskyBuiltinStaticTypeId> for StaticTypeId {
    fn from(id: HuskyBuiltinStaticTypeId) -> Self {
        Self::HuskyBuiltin(id)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum HuskyBuiltinStaticTypeId {
    Dataset,
    VirtualTy,
    VirtualVec,
}

// type level trait
pub trait AnyValue<'eval>:
    Debug + Send + Sync + Sized + PartialEq + Clone + RefUnwindSafe + 'eval
{
    fn static_type_id() -> StaticTypeId;
    fn static_type_name() -> Cow<'static, str>;
    // fn clone_shared(&self) -> Arc<dyn AnyValueDyn<'eval>>;

    fn clone_into_box(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(self.clone())
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        Arc::new(self.clone())
    }

    fn from_stack<'stack>(stack_value: StackValue<'stack, 'eval>) -> Self {
        match stack_value {
            StackValue::Owned(boxed_value) => boxed_value.take().unwrap(),
            _ => {
                p!(Self::static_type_name());
                p!(stack_value);
                panic!()
            }
        }
    }

    fn as_copyable(&self) -> CopyableValue {
        p!(self);
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:?}", self)
    }
}

// object safe trait
pub trait AnyValueDyn<'eval>: Debug + Send + Sync + RefUnwindSafe + 'eval {
    fn static_type_id_dyn(&self) -> StaticTypeId;
    fn static_type_name_dyn(&self) -> Cow<'static, str>;
    fn clone_into_box_dyn(&self) -> Box<dyn AnyValueDyn<'eval>>;
    fn clone_into_arc_dyn(&self) -> Arc<dyn AnyValueDyn<'eval>>;
    fn equal_any(&self, other: &dyn AnyValueDyn<'eval>) -> bool;
    fn assign<'stack>(&mut self, other: StackValue<'stack, 'eval>);
    fn primitive(&self) -> CopyableValue;
    fn upcast_any(&self) -> &(dyn AnyValueDyn<'eval> + 'eval);
    fn print_short(&self) -> String;
}

impl<'eval> dyn AnyValueDyn<'eval> {
    #[inline]
    pub fn downcast_ref<T: AnyValue<'eval>>(&self) -> &T {
        if T::static_type_id() != self.static_type_id_dyn() {
            panic!()
        }
        let ptr: *const dyn AnyValueDyn = &*self;
        let ptr: *const T = ptr as *const T;
        unsafe { &*ptr }
    }

    #[inline]
    pub fn downcast_mut<T: AnyValue<'eval>>(&mut self) -> &mut T {
        if T::static_type_id() != self.static_type_id_dyn() {
            p!(T::static_type_id(), self.static_type_id_dyn());
            panic!()
        }
        let ptr: *mut dyn AnyValueDyn = &mut *self;
        let ptr: *mut T = ptr as *mut T;
        unsafe { &mut *ptr }
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValueDyn<'eval> for T {
    fn static_type_id_dyn(&self) -> StaticTypeId {
        T::static_type_id().into()
    }

    fn static_type_name_dyn(&self) -> Cow<'static, str> {
        T::static_type_name()
    }

    fn clone_into_box_dyn(&self) -> Box<dyn AnyValueDyn<'eval>> {
        T::clone_into_box(self)
    }

    fn clone_into_arc_dyn(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        T::clone_into_arc(self)
    }

    fn equal_any(&self, other: &dyn AnyValueDyn) -> bool {
        todo!()
    }

    fn assign<'stack>(&mut self, other: StackValue<'stack, 'eval>) {
        *self = T::from_stack(other)
    }

    fn primitive(&self) -> CopyableValue {
        T::as_copyable(self)
    }

    fn upcast_any(&self) -> &dyn AnyValueDyn<'eval> {
        self
    }
    fn print_short(&self) -> String {
        T::print_short(self)
    }
}

impl<'eval> AnyValue<'eval> for i32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "i32".into()
    }

    fn clone_into_box(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        (*self).into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Copyable(CopyableValue::I32(value)) => value,
            StackValue::Owned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
}

impl<'eval> AnyValue<'eval> for f32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "f32".into()
    }

    fn clone_into_box(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Copyable(CopyableValue::F32(value)) => value,
            StackValue::Owned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
}

impl<'eval> AnyValue<'eval> for u32 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u32".into()
    }

    fn clone_into_box(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Copyable(CopyableValue::B32(value)) => value,
            StackValue::Owned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:#032b}", self)
    }
}

impl<'eval> AnyValue<'eval> for u64 {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "u64".into()
    }

    fn clone_into_box(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Copyable(CopyableValue::B64(value)) => value,
            StackValue::Owned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{:#064b}", self)
    }
}

impl<'eval> AnyValue<'eval> for bool {
    fn static_type_id() -> StaticTypeId {
        TypeId::of::<Self>().into()
    }

    fn static_type_name() -> Cow<'static, str> {
        "bool".into()
    }

    fn clone_into_box(&self) -> Box<dyn AnyValueDyn<'eval>> {
        Box::new(*self)
    }

    fn as_copyable(&self) -> CopyableValue {
        self.into()
    }

    fn from_stack(stack_value: StackValue) -> Self {
        match stack_value {
            StackValue::Copyable(CopyableValue::Bool(value)) => value,
            StackValue::Owned(boxed_value) => boxed_value.take().unwrap(),
            _ => panic!(),
        }
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
}

impl<'eval, T: AnyValue<'eval>> AnyValue<'eval> for Vec<T> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::VecOf(Box::new(T::static_type_id()))
    }

    fn static_type_name() -> Cow<'static, str> {
        todo!()
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }
}

impl<'eval> AnyValue<'eval> for Vec<MemberValue<'eval>> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::HuskyBuiltin(HuskyBuiltinStaticTypeId::VirtualVec)
    }

    fn static_type_name() -> Cow<'static, str> {
        "Vec".into()
    }

    fn clone_into_arc(&self) -> Arc<dyn AnyValueDyn<'eval>> {
        panic!()
    }

    fn print_short(&self) -> String {
        format!("{{ len: {}, data: [...] }}", self.len(),)
    }
}