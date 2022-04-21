use crate::*;

#[derive(Debug, Clone)]
pub enum MemberValue<'eval> {
    Primitive(PrimitiveValue),
    Boxed(BoxedValue<'eval>),
    GlobalPure(Arc<dyn AnyValueDyn<'eval>>),
    GlobalRef(&'eval dyn AnyValueDyn<'eval>),
    Moved,
}

impl<'eval> PartialEq for MemberValue<'eval> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Primitive(l0), Self::Primitive(r0)) => l0 == r0,
            (Self::Boxed(l0), Self::Boxed(r0)) => l0 == r0,
            (Self::GlobalPure(l0), Self::GlobalPure(r0)) => todo!(),
            (Self::GlobalRef(l0), Self::GlobalRef(r0)) => todo!(),
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

impl<'eval> Eq for MemberValue<'eval> {}

impl<'stack, 'eval: 'stack> MemberValue<'eval> {
    pub fn into_stack(self) -> StackValue<'stack, 'eval> {
        match self {
            MemberValue::Primitive(value) => StackValue::Primitive(value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn stack_ref(&self) -> StackValue<'stack, 'eval> {
        match self {
            MemberValue::Primitive(value) => StackValue::Primitive(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }

    pub fn share_globally(&self) -> EvalValue<'eval> {
        match self {
            MemberValue::Primitive(value) => EvalValue::Primitive(*value),
            MemberValue::Boxed(_) => todo!(),
            MemberValue::GlobalPure(_) => todo!(),
            MemberValue::GlobalRef(_) => todo!(),
            MemberValue::Moved => todo!(),
        }
    }
}