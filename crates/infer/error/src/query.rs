use ast::AstError;
use text::BindTextRange;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InferQueryError {
    pub kind: InferQueryErrorKind,
    pub message: String,
    pub dev_src: DevSource,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InferQueryErrorKind {
    Derived,
    Original,
}

pub type InferQueryResultArc<T> = Result<Arc<T>, InferQueryError>;
pub type InferQueryResult<T> = Result<T, InferQueryError>;

impl BindTextRange for InferQueryError {
    type Target = InferError;

    fn ref_bind_text_range(&self, range: TextRange) -> Self::Target {
        todo!()
    }

    fn bind_text_range(self, range: TextRange) -> Self::Target {
        InferError {
            variant: match self.kind {
                InferQueryErrorKind::Derived => InferErrorVariant::Derived {
                    message: self.message,
                },
                InferQueryErrorKind::Original => InferErrorVariant::Original {
                    message: self.message,
                    range,
                },
            },
            dev_src: self.dev_src,
        }
    }
}

impl From<EntitySyntaxError> for InferQueryError {
    fn from(e: EntitySyntaxError) -> Self {
        Self {
            kind: InferQueryErrorKind::Derived,
            message: e.message,
            dev_src: e.dev_src,
        }
    }
}

impl From<&AstError> for InferQueryError {
    fn from(e: &AstError) -> Self {
        Self {
            kind: InferQueryErrorKind::Derived,
            message: e.message(),
            dev_src: e.dev_src.clone(),
        }
    }
}

#[macro_export]
macro_rules! query_error {
    ($msg: expr) => {{
        InferQueryError {
            message: $msg,
            kind: InferQueryErrorKind::Original,
            dev_src: dev_utils::dev_src!(),
        }
    }};
}

#[macro_export]
macro_rules! query_derived_not_none {
    ($opt_value: expr) => {{
        $opt_value.ok_or(infer_error::InferQueryError {
            kind: infer_error::InferQueryErrorKind::Derived,
            message: "expect not none".to_string(),
            dev_src: dev_utils::dev_src!(),
        })
    }};
}

#[macro_export]
macro_rules! query_derived_ok {
    ($result: expr) => {{
        $result.or(infer_error::InferQueryError {
            variant: infer_error::InferQueryErrorKind::Derived,
            message: "expect ok".to_string(),
            dev_src: dev_utils::dev_src!(),
        })
    }};
}