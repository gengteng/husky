mod builder;
mod eager;
mod lazy;
mod query;
mod sheet;

pub use eager::*;
pub use husky_liason_semantics::*;
pub use lazy::*;
pub use query::*;
pub use sheet::*;

use husky_ast::RawExprIdx;
use husky_check_utils::*;
use husky_entity_syntax::EntitySyntaxResultArc;
use husky_file::FilePtr;
use husky_infer_entity_route::InferEntityRouteQueryGroup;
use husky_liason_semantics::*;
use husky_opn_syntax::*;
use husky_print_utils::*;
use infer_error::InferResult;

pub trait InferContract {
    fn contract_sheet(&self) -> &ContractSheet;

    fn lazy_expr_contract(&self, raw_expr_idx: RawExprIdx) -> InferResult<LazyContract> {
        self.contract_sheet().lazy_expr_contract(raw_expr_idx)
    }

    fn eager_expr_contract(&self, raw_expr_idx: RawExprIdx) -> InferResult<EagerContract> {
        self.contract_sheet().eager_expr_contract(raw_expr_idx)
    }
}