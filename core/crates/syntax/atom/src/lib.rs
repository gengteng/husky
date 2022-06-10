pub mod context;
mod convexity;
mod error;
pub mod parser;
mod stack;
mod variant;

pub use context::*;
pub use error::*;
pub use parser::AtomParser;
pub use parser::*;
use print_utils::p;
pub use variant::{AtomVariant, LambdaHead};

use entity_kind::*;
use entity_route::EntityRoutePtr;
use liason::*;
use text::TextRange;
use text::TextRanged;
use token::{Token, TokenKind};
use word::Identifier;
use word::RootIdentifier;

#[derive(Clone, PartialEq, Eq)]
pub struct Atom {
    pub range: TextRange,
    pub kind: AtomVariant,
}

impl Atom {
    pub fn new(range: TextRange, kind: AtomVariant) -> Atom {
        Atom { range, kind }
    }
}

impl TextRanged for Atom {
    fn text_range(&self) -> TextRange {
        self.range
    }
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Atom{{range: {:?}, kind: {:?}}}",
            &self.range, &self.kind
        ))
    }
}

impl From<&Token> for Atom {
    fn from(token: &Token) -> Self {
        match token.kind {
            TokenKind::Keyword(_) | TokenKind::Identifier(_) => {
                p!(token.kind);
                panic!()
            }
            TokenKind::Special(special) => Atom::new(token.text_range(), special.into()),
            TokenKind::PrimitiveLiteral(i) => Atom::new(token.text_range(), i.into()),
            TokenKind::WordOpr(word_opr) => Atom::new(token.text_range(), word_opr.into()),
            TokenKind::Unrecognized(_) => todo!(),
            TokenKind::IllFormedLiteral(_) => todo!(),
            TokenKind::Decorator(_) => todo!(),
        }
    }
}