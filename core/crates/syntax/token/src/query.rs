use crate::{line_token_iter::LineTokenIter, *};

use dev_utils::dev_src;
use file::{FileError, FileErrorKind, FileResultArc};
use print_utils::epin;
#[salsa::query_group(TokenQueryGroupStorage)]
pub trait TokenSalsaQueryGroup: file::FileQueryGroup + word::InternWord {
    fn tokenized_text(&self, id: file::FilePtr) -> FileResultArc<TokenizedText>;
}

fn tokenized_text(
    this: &dyn TokenSalsaQueryGroup,
    id: file::FilePtr,
) -> FileResultArc<TokenizedText> {
    if let Some(text) = this.raw_text(id) {
        return Ok(TokenizedText::parse(this.word_allocator(), text.as_str()));
    } else {
        Err(FileError {
            kind: FileErrorKind::FileNotFound,
            dev_src: dev_src!(),
        })
    }
}

pub trait TokenQueryGroup: TokenSalsaQueryGroup {
    fn tokenize(&self, line: &str) -> Vec<Token> {
        LineTokenIter::new(
            self.word_allocator(),
            0,
            line.chars().enumerate().peekable(),
        )
        .1
        .collect()
    }
}