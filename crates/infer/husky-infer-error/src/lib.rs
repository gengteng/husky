mod query;

use husky_vm::VMCompileError;
pub use query::*;

use std::fmt::Write;
use std::sync::Arc;

#[derive(Clone, PartialEq, Eq)]
pub struct InferError {
    pub variant: InferErrorVariant,
    pub dev_src: DevSource,
}

impl HuskyDisplay for InferError {
    fn write_inherent(&self, config: HuskyDisplayConfig, result: &mut String) {
        let message = match self.variant {
            InferErrorVariant::Derived { ref message } => message,
            InferErrorVariant::Original { ref message, .. } => message,
        };
        match config.colored {
            true => write!(
                result,
                "{}InferError{}: {}",
                husky_print_utils::RED,
                husky_print_utils::RESET,
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

impl From<&husky_ast::AstError> for InferError {
    fn from(error: &husky_ast::AstError) -> Self {
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

// impl BindTextRangeFrom<VMCompileError> for InferError {
//     fn bind_text_range_from(e: VMCompileError, range: TextRange) -> Self {
//         Self {
//             variant: InferErrorVariant::Original {
//                 message: e.message,
//                 range,
//             },
//             dev_src: e.dev_src,
//         }
//     }
//     fn bind_text_range_from_ref(e: &VMCompileError, range: TextRange) -> Self {
//         Self {
//             variant: InferErrorVariant::Original {
//                 message: e.message.clone(),
//                 range,
//             },
//             dev_src: e.dev_src.clone(),
//         }
//     }
// }

#[macro_export]
macro_rules! error {
    ($msg:expr, $range: expr) => {{
        husky_infer_error::InferError {
            variant: husky_infer_error::InferErrorVariant::Original {
                message: $msg.into(),
                range: $range,
            },
            dev_src: husky_dev_utils::dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! throw {
    ($msg:expr, $range: expr) => {{
        Err(husky_infer_error::InferError {
            variant: husky_infer_error::InferErrorVariant::Original {
                message: $msg.into(),
                range: $range,
            },
            dev_src: husky_dev_utils::dev_src!(),
        })?
    }};
}

#[macro_export]
macro_rules! throw_derived {
    ($msg:expr) => {{
        Err(husky_infer_error::InferError {
            variant: husky_infer_error::InferErrorVariant::Derived {
                message: $msg.into(),
            },
            dev_src: husky_dev_utils::dev_src!(),
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
            dev_src: husky_dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! derived_not_none {
    ($opt_value: expr) => {{
        $opt_value.ok_or(husky_infer_error::InferError {
            variant: husky_infer_error::InferErrorVariant::Derived {
                message: "expect not none".to_string(),
            },
            dev_src: husky_dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! derived {
    ($message: expr) => {{
        husky_infer_error::InferError {
            variant: husky_infer_error::InferErrorVariant::Derived {
                message: $message.into(),
            },
            dev_src: husky_dev_utils::dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! derived_unwrap {
    ($result: expr) => {{
        $result.map_err(|e| husky_infer_error::InferError {
            variant: husky_infer_error::InferErrorVariant::Derived {
                message: format!("expect ok but got {e:?} instead"),
            },
            dev_src: husky_dev_utils::dev_src!(),
        })?
    }};
}

use husky_dev_utils::*;
use husky_display_utils::{HuskyDisplay, HuskyDisplayConfig};
use husky_entity_syntax::EntitySyntaxError;
use husky_text::TextRange;
