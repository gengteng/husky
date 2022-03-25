mod error;
mod kind;
mod line_token_iter;
mod query;
mod scanner;
mod special;
#[cfg(test)]
mod tests;
mod tokenized_text;

pub use error::LexError;
pub use kind::TokenKind;
pub use query::{TokenQueryGroup, TokenQueryGroupStorage};
pub use special::Special;
pub use tokenized_text::{TokenGroupIter, TokenizedText};

use scanner::TokenScanner;
use text::{TextIndent, TextRange, TextRanged};

#[derive(PartialEq, Eq)]
pub struct Token {
    pub range: TextRange,
    pub kind: TokenKind,
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("Token {{{:?}, {:?}}}", self.kind, self.range))
    }
}

impl Token {
    pub fn new(i: usize, start: usize, end: usize, kind: TokenKind) -> Token {
        Token {
            range: text::new_same_line(i, start, end),
            kind,
        }
    }
}

impl TextRanged for Token {
    fn text_range_ref(&self) -> &TextRange {
        &self.range
    }
}