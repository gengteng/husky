mod field;
mod index;
mod method_elem;

use std::panic::catch_unwind;

use super::*;

#[derive(Clone, Copy)]
pub struct __LinkageFp {
    pub wrapper: for<'eval> unsafe fn(
        Option<&dyn __EvalContext<'eval>>,
        &mut [__Register<'eval>],
    ) -> __Register<'eval>,
    pub opt_fp: Option<*const ()>,
}

#[cfg(feature = "extra")]
impl __LinkageFp {
    pub fn eval<'eval>(
        self,
        opt_ctx: Option<&dyn __EvalContext<'eval>>,
        mut arguments: Vec<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        catch_unwind(move || unsafe { (self.wrapper)(opt_ctx, &mut arguments).into_eval() })
            .map_err(|e| {
                todo!()
                // EvalError::Normal {
                //     message: format!(
                //         "error: {e:?} when calling linkage with src = {}",
                //         self.dev_src
                //     ),
                // }
            })
    }

    pub fn call<'eval>(
        self,
        opt_ctx: Option<&dyn __EvalContext<'eval>>,
        mut arguments: Vec<__Register<'eval>>,
    ) -> __VMResult<__Register<'eval>> {
        catch_unwind(move || unsafe { (self.wrapper)(opt_ctx, &mut arguments) }).map_err(|e| {
            todo!()
            // EvalError::Normal {
            //     message: format!(
            //         "error: {e:?} when calling linkage with src = {}",
            //         self.dev_src
            //     ),
            // }
        })
    }
}

impl std::fmt::Debug for __LinkageFp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("__LinkageFp")
            .field("wrapper", &(self.wrapper as *const ()))
            .field("opt_fp", &self.opt_fp)
            .finish()
    }
}
impl PartialEq for __LinkageFp {
    fn eq(&self, other: &Self) -> bool {
        self.wrapper as usize == other.wrapper as usize && self.opt_fp == other.opt_fp
    }
}
impl Eq for __LinkageFp {}
unsafe impl Send for __LinkageFp {}
unsafe impl Sync for __LinkageFp {}

#[macro_export]
macro_rules! linkage_fp {
    ($wrapper: expr, some $raw_fp: expr) => {{
        __LinkageFp {
            wrapper: $wrapper,
            opt_fp: Some($raw_fp as *const ()),
        }
    }};
    ($wrapper: expr, none) => {{
        __LinkageFp {
            wrapper: $wrapper,
            opt_fp: None,
        }
    }};
    ($wrapper: expr) => {{
        __LinkageFp {
            wrapper: $wrapper,
            opt_fp: None,
        }
    }};
}