macro_rules! identify {
    ($file: expr, $token:expr) => {{
        match $token.kind {
            TokenKind::Identifier(Identifier::Custom(ident)) => ident,
            _ => err!($file, $token.range, "expect `<custom_identifier>`")?,
        }
    }};
}
pub(super) use identify;

macro_rules! expect {
    ($cond:expr, $file: expr, $range:expr, $msg:expr) => {
        if !$cond {
            err!($file, $range, $msg)?
        }
    };
}
pub(super) use expect;

macro_rules! expect_kind {
    ($file: expr, $token:expr, $kind:expr) => {
        expect!(
            $token.kind == TokenKind::Special($kind),
            $file,
            $token.range,
            format!("expect `{}`", $kind.code())
        );
    };
}
pub(super) use expect_kind;

macro_rules! expect_block_head {
    ($file: expr, $tokens:expr) => {
        expect_kind!($file, $tokens.last().unwrap(), Special::Colon)
    };
}
pub(super) use expect_block_head;

macro_rules! expect_at_least {
    ($tokens:expr, $file: expr, $kw_range:expr, $lower_bound:expr) => {
        expect!(
            $tokens.len() >= $lower_bound,
            $file,
            $kw_range,
            format!(
                "expect at least {} tokens after keyword, but got {} tokens instead",
                $lower_bound,
                $tokens.len()
            )
        );
    };
}
pub(super) use expect_at_least;

macro_rules! expect_len {
    ($file: expr, $tokens:expr,  $len:expr) => {
        expect!(
            $tokens.len() == $len,
            $file,
            $tokens.into(),
            format!(
                "expect {} tokens after keyword, but got {} tokens instead",
                $len,
                $tokens.len()
            )
        );
    };
}
pub(super) use expect_len;

macro_rules! trim_colon {
    ($file: expr, $tokens:expr; keyword, colon) => {{
        expect_kind!($file, $tokens.last().unwrap(), Special::Colon);
        &$tokens[1..($tokens.len() - 1)]
    }};

    ($tokens:expr, $kw_range:expr) => {{
        expect_at_least!($tokens, $kw_range, 1);
        expect_kind!($tokens.last().unwrap(), Special::Colon);
        &$tokens[0..($tokens.len() - 1)]
    }};
}
pub(super) use trim_colon;

macro_rules! expect_head {
    ($file: expr, $tokens:expr) => {{
        expect_kind!($file, $tokens.last().unwrap(), Special::Colon);
    }};
}
pub(super) use expect_head;