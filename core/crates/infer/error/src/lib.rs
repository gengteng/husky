mod query;

pub use query::*;
use vm::VMCompileError;

use std::fmt::Write;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct InferError {
    pub variant: InferErrorVariant,
    pub dev_src: DevSource,
}

impl TestDisplay for InferError {
    fn write_inherent(&self, config: TestDisplayConfig, result: &mut String) {
        let message = match self.variant {
            InferErrorVariant::Derived { ref message } => message,
            InferErrorVariant::Original { ref message, range } => message,
        };
        match config.colored {
            true => write!(
                result,
                "{}InferError{}: {}",
                print_utils::RED,
                print_utils::RESET,
                message
            )
            .unwrap(),
            false => write!(result, "InferError: {}", message).unwrap(),
        }
    }
}

impl InferError {
    pub fn derived(&self) -> Self {
        Self {
            variant: InferErrorVariant::Derived {
                message: match self.variant {
                    InferErrorVariant::Derived { ref message } => message.clone(),
                    InferErrorVariant::Original { .. } => format!("{:?}", self),
                },
            },
            dev_src: self.dev_src.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferErrorVariant {
    Derived { message: String },
    Original { message: String, range: TextRange },
}

impl std::fmt::Debug for InferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // f.debug_struct("InferError")
        //     .field("message", &self.message)
        //     .field("src", &self.src)
        //     .finish()
        f.write_fmt(format_args!(
            "InferError:\n\
    src: {:?}\n\
    kind:\n\
{:?}",
            &self.dev_src, &self.variant
        ))
    }
}

impl InferError {
    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

pub type InferResult<T> = Result<T, InferError>;

pub type InferResultArc<T> = Result<Arc<T>, InferError>;
pub type InferResultArcRef<'a, T> = Result<&'a Arc<T>, InferError>;

impl From<EntitySyntaxError> for InferError {
    fn from(error: EntitySyntaxError) -> Self {
        Self {
            variant: InferErrorVariant::Derived {
                message: format!("ScopeError {:?}", error),
            },
            dev_src: error.dev_src,
        }
    }
}

impl From<&ast::AstError> for InferError {
    fn from(error: &ast::AstError) -> Self {
        Self {
            variant: InferErrorVariant::Derived {
                message: format!("{:?}", error),
            },
            dev_src: error.dev_src.clone(),
        }
    }
}

impl From<InferQueryError> for InferError {
    fn from(e: InferQueryError) -> Self {
        Self {
            variant: InferErrorVariant::Derived { message: e.message },
            dev_src: e.dev_src.clone(),
        }
    }
}

impl BindTextRangeFrom<VMCompileError> for InferError {
    fn bind_text_range_from(e: VMCompileError, range: TextRange) -> Self {
        Self {
            variant: InferErrorVariant::Original {
                message: e.message,
                range,
            },
            dev_src: e.dev_src,
        }
    }
    fn bind_text_range_from_ref(e: &VMCompileError, range: TextRange) -> Self {
        Self {
            variant: InferErrorVariant::Original {
                message: e.message.clone(),
                range,
            },
            dev_src: e.dev_src.clone(),
        }
    }
}

#[macro_export]
macro_rules! error {
    ($msg:expr, $range: expr) => {{
        infer_error::InferError {
            variant: infer_error::InferErrorVariant::Original {
                message: $msg.into(),
                range: $range,
            },
            dev_src: dev_utils::dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! throw {
    ($msg:expr, $range: expr) => {{
        Err(infer_error::InferError {
            variant: infer_error::InferErrorVariant::Original {
                message: $msg.into(),
                range: $range,
            },
            dev_src: dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! throw_derived {
    ($msg:expr) => {{
        Err(infer_error::InferError {
            variant: infer_error::InferErrorVariant::Derived {
                message: $msg.into(),
            },
            dev_src: dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! ok_or {
    ($opt_value: expr, $msg:expr, $range: expr) => {{
        $opt_value.ok_or(InferError {
            variant: InferErrorVariant::Original {
                message: $msg.into(),
                range: $range,
            },
            dev_src: dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! derived_not_none {
    ($opt_value: expr) => {{
        $opt_value.ok_or(infer_error::InferError {
            variant: infer_error::InferErrorVariant::Derived {
                message: "expect not none".to_string(),
            },
            dev_src: dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! derived {
    ($message: expr) => {{
        infer_error::InferError {
            variant: infer_error::InferErrorVariant::Derived {
                message: $message.into(),
            },
            dev_src: dev_utils::dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! derived_unwrap {
    ($result: expr) => {{
        $result.or(Err(infer_error::InferError {
            variant: infer_error::InferErrorVariant::Derived {
                message: "expect ok".to_string(),
            },
            dev_src: dev_utils::dev_src!(),
        }))?
    }};
}

use dev_utils::*;
use entity_syntax::EntitySyntaxError;
use test_utils::{TestDisplay, TestDisplayConfig};
use text::{BindTextRangeFrom, TextRange};