use std::borrow::Cow;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub enum CallFormSource {
    Func { stmts: Avec<FuncStmt> },
    Proc { stmts: Avec<ProcStmt> },
    Lazy { stmts: Avec<LazyStmt> },
    Static(Linkage),
}

impl From<Linkage> for CallFormSource {
    fn from(linkage: Linkage) -> Self {
        CallFormSource::Static(linkage)
    }
}