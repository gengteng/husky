use crate::*;

use super::*;

pub struct BoxedValue<'eval> {
    inner: Box<dyn AnyValueDyn + 'eval>,
}

impl<'eval> BoxedValue<'eval> {
    pub fn new<T: AnyValueDyn + 'eval>(value: T) -> BoxedValue<'eval> {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn take<T: AnyValue>(self) -> VMResult<T> {
        // check type
        if (*self.inner).static_type_id() != T::static_type_id() {
            Err(VMError::TypeMismatch)
        } else {
            let raw_pointer: *const (dyn AnyValueDyn + 'eval) =
                Box::<(dyn AnyValueDyn + 'eval)>::into_raw(self.inner);
            Ok(unsafe { *Box::from_raw(raw_pointer as *mut T) })
        }
    }

    pub fn pointer(&self) -> *const (dyn AnyValueDyn + 'eval) {
        &*(self.inner)
    }

    pub fn as_ref(&self) -> &dyn AnyValueDyn {
        &*self.inner
    }
}

impl<'eval> std::fmt::Debug for BoxedValue<'eval> {
    fn fmt(&self, f: &mut common::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[test]
fn test_owned() {
    use common::*;

    let a = BoxedValue::new(0 as i32);
    let b: i32 = a.take().unwrap();
    should_eq!(b, 0);
}

#[test]
fn test_owned_type_mistach() {
    use common::*;

    let a = BoxedValue::new(0 as i32);
    let b = a.take::<f32>();
    should!(b.is_err());
}