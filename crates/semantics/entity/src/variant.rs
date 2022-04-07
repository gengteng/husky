mod morphism;
mod routine;
mod ty;

use ast::InputPlaceholder;
pub use morphism::*;
pub use routine::*;
use semantics_lazy::LazyStmt;
pub use ty::*;

use crate::*;
use entity_route::RangedEntityRoute;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityDefnVariant {
    Main(MainDefn),
    Module {},
    Feature {
        ty: RangedEntityRoute,
        lazy_stmts: Arc<Vec<Arc<LazyStmt>>>,
    },
    Pattern {},
    Func {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        stmts: Arc<Vec<Arc<FuncStmt>>>,
    },
    Proc {
        input_placeholders: Arc<Vec<InputPlaceholder>>,
        output: RangedEntityRoute,
        stmts: Arc<Vec<Arc<ProcStmt>>>,
    },
    Ty(TyDefn),
    EnumVariant(EnumVariant),
    Builtin,
}