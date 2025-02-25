use std::sync::Arc;

use husky_dev_utils::DevSource;
use husky_file::FileError;
use husky_text::TextRange;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AtomError {
    pub variant: AtomErrorVariant,
    pub dev_src: DevSource,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AtomErrorVariant {
    Original { message: String, range: TextRange },
    Derived,
}

macro_rules! error {
    ($message: expr, $range: expr) => {{
        AtomError {
            variant: AtomErrorVariant::Original {
                message: $message.into(),
                range: $range,
            },
            dev_src: husky_dev_utils::dev_src!(),
        }
    }};
}
pub(crate) use error;

macro_rules! err {
    ($message: expr, $range: expr) => {{
        Err(AtomError {
            variant: AtomErrorVariant::Original {
                message: $message.into(),
                range: $range,
            },
            dev_src: husky_dev_utils::dev_src!(),
        })
    }};
}
pub(crate) use err;

impl From<FileError> for AtomError {
    fn from(_: FileError) -> Self {
        todo!()
    }
}

impl From<husky_entity_syntax::ModuleFromFileError> for AtomError {
    fn from(_: husky_entity_syntax::ModuleFromFileError) -> Self {
        Self {
            variant: AtomErrorVariant::Derived,
            dev_src: husky_dev_utils::dev_src!(),
        }
    }
}

pub type AtomResult<T> = Result<T, AtomError>;
pub type AtomResultArc<T> = Result<Arc<T>, AtomError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AtomRule {
    BeforeColonShouldBeScope,
    ListShouldBeWellFormed,
    ScopeShouldExist,
    AfterColonShouldBeCustomIdentifier,
    NonEmptyAngles,
    ExpectCommaInAngle,
    AfterLAngleShouldBeCommaListOfScopes,
    KeywordShouldBeAtStart,
    CompatibleConvexity,
    ExpectTypeAfterLightArrow,
    BracketsShouldMatch,
    OnlyTypesInTheParenthesisBeforeLightArrow,
}
