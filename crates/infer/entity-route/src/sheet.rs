mod expr;
mod var;

use std::{collections::HashMap, sync::Arc};

use arena::map::ArenaMap;
use ast::{AstText, RawExpr};
use builder::EntityRouteSheetBuilder;
use fold::FoldStorage;
use infer_decl::MemberIdx;
use text::Row;
use word::CustomIdentifier;

use super::*;

pub(crate) fn entity_route_sheet(
    db: &dyn InferEntityRouteQueryGroup,
    file: FilePtr,
) -> EntityRouteResultArc<EntityRouteSheet> {
    let ast_text = db.ast_text(file)?;
    let mut ty_sheet_builder = EntityRouteSheetBuilder::new(db, ast_text.clone());
    ty_sheet_builder.infer_all(ast_text.folded_results.iter());
    Ok(Arc::new(ty_sheet_builder.finish()))
}

#[derive(Debug, PartialEq, Eq)]
pub struct EntityRouteSheet {
    pub ast_text: Arc<AstText>,
    pub(crate) expr_tys: RawExprMap<InferResult<EntityRoutePtr>>,
    pub(crate) call_routes: RawExprMap<InferResult<EntityRoutePtr>>,
    pub(crate) variable_tys: HashMap<(CustomIdentifier, Row), EntityRoutePtr>,
    pub(crate) extra_errors: Vec<InferError>,
}

impl EntityRouteSheet {
    pub(crate) fn new(ast_text: Arc<AstText>, extra_errors: Vec<InferError>) -> Self {
        Self {
            expr_tys: ArenaMap::new(&ast_text.arena),
            call_routes: ArenaMap::new(&ast_text.arena),
            variable_tys: Default::default(),
            ast_text,
            extra_errors,
        }
    }

    pub fn expr_ty_result(&self, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        if let Some(ref expr_ty) = self.expr_tys.get(expr_idx) {
            match expr_ty {
                Ok(ty) => Ok(*ty),
                Err(e) => Err(e.derived()),
            }
        } else {
            p!(self.expr_tys);
            p!(self.ast_text.file);
            p!(self.ast_text.arena);
            p!(expr_idx);
            panic!()
        }
    }

    pub fn call_route(&self, expr_idx: RawExprIdx) -> InferResult<EntityRoutePtr> {
        match derived_not_none!(self.call_routes.get(expr_idx))? {
            Ok(call_route) => Ok(*call_route),
            Err(e) => Err(e.derived()),
        }
    }

    pub fn original_errors(&self) -> Vec<&InferError> {
        let mut errors: Vec<&InferError> = self
            .extra_errors
            .iter()
            .filter_map(|e| match e.variant {
                InferErrorVariant::Derived { .. } => None,
                InferErrorVariant::Original { .. } => Some(e),
            })
            .collect();
        for result in self.expr_tys.values() {
            match result {
                Ok(_) => (),
                Err(error) => match error.variant {
                    InferErrorVariant::Derived { .. } => (),
                    InferErrorVariant::Original { .. } => errors.push(error),
                },
            }
        }

        errors
    }
}
