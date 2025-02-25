use crate::*;
use husky_ast::*;
use husky_context_impls::ReturnContext;
use husky_entity_route::RangedEntityRoute;
use husky_file::FilePtr;
use husky_infer_entity_route::{EntityRouteSheet, InferEntityRoute};
use husky_infer_qualified_ty::{InferQualifiedTy, QualifiedTySheet};
use husky_opn_semantics::ImplicitConversion;
use husky_semantics_error::*;
use husky_word::{CustomIdentifier, IdentPairDict, RootIdentifier};
use husky_xml_syntax::XmlTagKind;
use infer_contract::{ContractSheet, InferContract};
use std::{iter::Peekable, sync::Arc};

pub(super) struct LazyStmtParser<'a> {
    pub(super) db: &'a dyn InferQueryGroup,
    pub(super) arena: &'a RawExprArena,
    pub(super) file: FilePtr,
    entity_route_sheet: Arc<EntityRouteSheet>,
    contract_sheet: Arc<ContractSheet>,
    qualified_ty_sheet: Arc<QualifiedTySheet>,
}

impl<'a> LazyStmtParser<'a> {
    pub(super) fn new(db: &'a dyn InferQueryGroup, arena: &'a RawExprArena, file: FilePtr) -> Self {
        Self {
            db,
            arena,
            file,
            entity_route_sheet: db.entity_route_sheet(file).unwrap(),
            contract_sheet: db.contract_sheet(file).unwrap(),
            qualified_ty_sheet: db.qualified_ty_sheet(file).unwrap(),
        }
    }

    pub(super) fn parse_lazy_stmts(
        &mut self,
        iter: AstIter,
        output_ty: RangedEntityRoute,
    ) -> SemanticResultArc<Vec<Arc<LazyStmt>>> {
        let mut stmts = Vec::new();
        let mut iter = iter.peekable();
        while let Some(item) = iter.next() {
            match item.value.as_ref().unwrap().variant {
                AstVariant::Use { .. } => todo!(),
                AstVariant::Stmt(ref stmt) => {
                    let variant = match stmt.variant {
                        RawStmtVariant::Loop(_) => panic!(),
                        RawStmtVariant::ConditionBranch {
                            condition_branch_kind,
                        } => self.parse_condition_flow(
                            not_none!(item.opt_children),
                            &mut iter,
                            condition_branch_kind,
                            output_ty,
                        )?,
                        RawStmtVariant::PatternBranch { .. } => panic!(),
                        RawStmtVariant::Exec { .. } => todo!(),
                        RawStmtVariant::Init {
                            varname,
                            initial_value,
                            init_kind: kind,
                        } => {
                            let initial_value = self.parse_lazy_expr(initial_value, None)?;
                            if kind != InitKind::Decl {
                                todo!()
                            }
                            LazyStmtVariant::Init {
                                varname,
                                value: initial_value,
                            }
                        }
                        RawStmtVariant::Return {
                            result,
                            return_context,
                        } => match self.arena[result].variant {
                            RawExprVariant::Opn {
                                opn_variant: RawOpnVariant::Suffix(RawSuffixOpr::Unveil),
                                ref opds,
                            } => {
                                let result = self.parse_lazy_expr(opds.start, None)?;
                                let implicit_conversion = ImplicitConversion::from_opt_expectation(
                                    Some(
                                        self.db.opt_ty(
                                            self.db
                                                .implement_target(return_context.return_ty())
                                                .unwrap(),
                                        ),
                                    ),
                                    &result.qualified_ty,
                                );
                                LazyStmtVariant::ReturnUnveil {
                                    result,
                                    implicit_conversion,
                                    return_context: ReturnContext::from_raw(
                                        self.db.upcast(),
                                        return_context,
                                    ),
                                }
                            }
                            _ => LazyStmtVariant::Return {
                                result: self.parse_lazy_expr(
                                    result,
                                    Some(
                                        self.db
                                            .implement_target(return_context.return_ty())
                                            .unwrap(),
                                    ),
                                )?,
                            },
                        },
                        RawStmtVariant::Assert(condition) => LazyStmtVariant::Assert {
                            condition: self
                                .parse_lazy_expr(condition, Some(RootIdentifier::Bool.into()))?,
                        },
                        RawStmtVariant::Require {
                            condition,
                            return_context,
                        } => LazyStmtVariant::Require {
                            condition: self
                                .parse_lazy_expr(condition, Some(RootIdentifier::Bool.into()))?,
                            return_context: ReturnContext::from_raw(
                                self.db.upcast(),
                                return_context,
                            ),
                        },
                        RawStmtVariant::Break => todo!(),
                        RawStmtVariant::Match { .. } => panic!(),
                        RawStmtVariant::ReturnXml(ref raw_xml_expr) => LazyStmtVariant::ReturnXml {
                            xml_expr: self.parse_xml_expr(raw_xml_expr)?,
                        },
                    };
                    stmts.push(Arc::new(LazyStmt {
                        file: self.file,
                        range: stmt.range,
                        indent: item.indent,
                        variant,
                        instruction_id: Default::default(),
                        output_ty,
                    }))
                }
                _ => panic!("Unexpected"),
            }
        }
        Ok(Arc::new(stmts))
    }

    fn parse_xml_expr(&mut self, raw_xml_expr: &RawXmlExpr) -> SemanticResultArc<XmlExpr> {
        let variant = match raw_xml_expr.variant {
            RawXmlExprVariant::Value(raw_expr_idx) => {
                XmlExprVariant::Value(self.parse_lazy_expr(raw_expr_idx, None)?)
            }
            RawXmlExprVariant::Tag { ident, ref props } => {
                let tag_kind = XmlTagKind::from_ident(ident);
                XmlExprVariant::Tag { tag_kind, props: props
                    .iter()
                    .map(
                        |(ident, raw_expr_idx)| -> SemanticResult<(CustomIdentifier, Arc<LazyExpr>)> {
                            Ok((*ident, self.parse_lazy_expr(*raw_expr_idx, None)?))
                        },
                    )
                    .collect::<SemanticResult<IdentPairDict<Arc<LazyExpr>>>>()? }
            }
        };
        Ok(Arc::new(XmlExpr {
            variant,
            range: raw_xml_expr.range,
            file: self.file,
            instruction_id: Default::default(),
        }))
    }

    fn parse_condition_flow(
        &mut self,
        children: AstIter,
        iter: &mut Peekable<AstIter>,
        condition_branch_kind: RawConditionBranchKind,
        ty: RangedEntityRoute,
    ) -> SemanticResult<LazyStmtVariant> {
        let mut branches = vec![];
        match condition_branch_kind {
            RawConditionBranchKind::If { condition } => {
                branches.push(Arc::new(LazyConditionBranch {
                    variant: LazyConditionBranchVariant::If {
                        condition: self.parse_lazy_expr(condition, None)?,
                    },
                    stmts: self.parse_lazy_stmts(children, ty)?,
                }))
            }
            RawConditionBranchKind::Elif { .. } => todo!(),
            RawConditionBranchKind::Else => todo!(),
        }
        while let Some(item) = iter.peek() {
            let item = match item.value.as_ref().unwrap().variant {
                AstVariant::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::ConditionBranch {
                            condition_branch_kind: RawConditionBranchKind::If { .. },
                            ..
                        },
                    ..
                }) => break,
                AstVariant::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::ConditionBranch {
                            condition_branch_kind,
                            ..
                        },
                    ..
                }) => {
                    if matches!(condition_branch_kind, RawConditionBranchKind::If { .. }) {
                        break;
                    }
                    iter.next().unwrap()
                }
                _ => break,
            };
            match item.value.as_ref().unwrap().variant {
                AstVariant::Stmt(RawStmt {
                    variant:
                        RawStmtVariant::ConditionBranch {
                            ref condition_branch_kind,
                        },
                    ..
                }) => match condition_branch_kind {
                    RawConditionBranchKind::If { .. } => panic!(),
                    RawConditionBranchKind::Elif { .. } => {
                        if branches.len() == 0 {
                            todo!()
                        }
                        todo!()
                    }
                    RawConditionBranchKind::Else => {
                        branches.push(Arc::new(LazyConditionBranch {
                            variant: LazyConditionBranchVariant::Else,
                            stmts: self.parse_lazy_stmts(not_none!(item.opt_children), ty)?,
                        }));
                        break;
                    }
                },
                _ => panic!(),
            }
        }
        Ok(LazyStmtVariant::ConditionFlow { branches, ty })
    }
}

impl<'a> InferEntityRoute for LazyStmtParser<'a> {
    fn decl_db(&self) -> &dyn infer_decl::DeclQueryGroup {
        self.db.upcast()
    }

    fn entity_route_sheet(&self) -> &EntityRouteSheet {
        &self.entity_route_sheet
    }
}

impl<'a> InferContract for LazyStmtParser<'a> {
    fn contract_sheet(&self) -> &ContractSheet {
        &self.contract_sheet
    }
}

impl<'a> InferQualifiedTy for LazyStmtParser<'a> {
    fn qualified_ty_sheet(&self) -> &husky_infer_qualified_ty::QualifiedTySheet {
        &self.qualified_ty_sheet
    }
}

impl<'a> LazyExprParser<'a> for LazyStmtParser<'a> {
    fn arena(&self) -> &'a RawExprArena {
        self.arena
    }

    fn db(&self) -> &'a dyn InferQueryGroup {
        self.db
    }

    fn file(&self) -> FilePtr {
        self.file
    }
}
