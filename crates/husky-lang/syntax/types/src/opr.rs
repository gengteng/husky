use scope::ScopeId;
use word::Identifier;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Opr {
    Binary(BinaryOpr),
    Prefix(PrefixOpr),
    Suffix(SuffixOpr),
    List(ListOpr),
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ListOpr {
    TupleInit,
    NewVec,
    NewDict,
    Call,
    Index,
    ModuloIndex,
    StructInit,
}

impl From<BinaryOpr> for Opr {
    fn from(binary: BinaryOpr) -> Self {
        Self::Binary(binary)
    }
}

impl From<PrefixOpr> for Opr {
    fn from(prefix: PrefixOpr) -> Self {
        Self::Prefix(prefix)
    }
}

impl From<SuffixOpr> for Opr {
    fn from(suffix: SuffixOpr) -> Self {
        Self::Suffix(suffix)
    }
}

impl From<ListOpr> for Opr {
    fn from(list: ListOpr) -> Self {
        Self::List(list)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SuffixOpr {
    Incr,                     // ++
    Decr,                     // --
    MayReturn,                // ?
    MemberAccess(Identifier), // .
    WithType(ScopeId),        // :
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum PrefixOpr {
    Minus,     // -
    Not,       // !$0
    BitNot,    // ~
    Shared,    // &
    Exclusive, // !$0 after WithType or Vec or Array
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bracket {
    Par,
    Box,
    Curl,
}

impl Bracket {
    pub fn bra_code(&self) -> &'static str {
        match self {
            Bracket::Par => "(",
            Bracket::Box => "[",
            Bracket::Curl => "{",
        }
    }

    pub fn ket_code(&self) -> &'static str {
        match self {
            Bracket::Par => ")",
            Bracket::Box => "]",
            Bracket::Curl => "}",
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListStartAttr {
    None,
    Attach,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ListEndAttr {
    None,
    Attach,
    Modulo,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum JoinOpr {
    Comma, // ,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BinaryOpr {
    Less,    // <
    Leq,     // <=
    Greater, // >
    Geq,     // >=
    Neq,     // !=
    Eq,      // ==
    LShift,  // >>
    RShift,  // <<
    Add,     // +
    Sub,     // -
    Mult,    // *
    Div,     // /
    Power,   // **
    And,     // && and
    BitAnd,  // &
    Or,      // ||
    BitXor,  // ^
    BitOr,   // |
    Modulo,  // %
}
impl BinaryOpr {
    pub fn spaced_code(&self) -> &'static str {
        match self {
            BinaryOpr::Less => " < ",
            BinaryOpr::Leq => " <= ",
            BinaryOpr::Greater => " > ",
            BinaryOpr::Geq => " >= ",
            BinaryOpr::Neq => " != ",
            BinaryOpr::Eq => " == ",
            BinaryOpr::LShift => " << ",
            BinaryOpr::RShift => " >>",
            BinaryOpr::Add => " + ",
            BinaryOpr::Sub => " - ",
            BinaryOpr::Mult => " * ",
            BinaryOpr::Div => " / ",
            BinaryOpr::Power => " ** ",
            BinaryOpr::And => " && ",
            BinaryOpr::BitAnd => " & ",
            BinaryOpr::Or => " || ",
            BinaryOpr::BitXor => " ^ ",
            BinaryOpr::BitOr => " | ",
            BinaryOpr::Modulo => " % ",
        }
    }
}