use std::iter::zip;

use defn_head::Parameter;
use entity_kind::EntityKind;
use husky_ast::*;
use husky_check_utils::should;
use husky_entity_route::{EntityRoutePtr, EntityRouteVariant};
use husky_opn_syntax::RawOpnVariant;
use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_print_utils::{epin, msg_once, p};
use husky_text::{BindTextRangeInto, RangedCustomIdentifier};
use husky_text::{TextRange, TextRanged};
use infer_error::{
    derived, derived_not_none, derived_unwrap, throw, throw_derived, InferError, InferErrorVariant,
};

use super::*;

impl<'a> QualifiedTySheetBuilder<'a> {
    pub(super) fn infer_eager_call_form(
        &mut self,
        inputs: &[Parameter],
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        self.add_eager_inputs(inputs);
        self.infer_eager_stmts(ast_iter, opt_output_ty, output_liason)
    }

    fn add_eager_inputs(&mut self, inputs: &[Parameter]) {
        for parameter in inputs {
            let ty = parameter.ranged_ty.route;
            self.qualified_ty_sheet
                .eager_variable_qualified_tys
                .insert_new((
                    (
                        parameter.ranged_ident.ident.into(),
                        parameter.ranged_ident.range,
                    ),
                    EagerVariableQualifiedTy::from_parameter(
                        self.db,
                        ty,
                        parameter.ranged_liason.liason,
                    ),
                ));
        }
    }

    fn infer_eager_stmts(
        &mut self,
        ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        for item in ast_iter.clone() {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => {
                        self.infer_eager_stmt(stmt, opt_output_ty, output_liason)
                    }
                    _ => (),
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_eager_stmts(children, opt_output_ty, output_liason)
            }
        }
    }

    fn infer_eager_stmt(
        &mut self,
        stmt: &RawStmt,
        opt_output_ty: Option<EntityRoutePtr>,
        output_liason: OutputLiason,
    ) {
        match stmt.variant {
            RawStmtVariant::Loop(loop_kind) => match loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    self.qualified_ty_sheet
                        .eager_variable_qualified_tys
                        .insert_new((
                            (frame_var.ident.into(), frame_var.range),
                            Ok(EagerVariableQualifiedTy {
                                qual: EagerVariableQualifier::Copyable,
                                ty: EntityRoutePtr::Root(RootIdentifier::I32),
                            }),
                        ));
                    if let Some(bound) = initial_boundary.opt_bound {
                        self.infer_eager_expr(bound);
                    }
                    if let Some(bound) = final_boundary.opt_bound {
                        self.infer_eager_expr(bound);
                    }
                }
                RawLoopKind::ForExt { final_boundary, .. } => {
                    if let Some(bound) = final_boundary.opt_bound {
                        self.infer_eager_expr(bound);
                    }
                }
                RawLoopKind::While { condition } => {
                    self.infer_eager_expr(condition);
                }
                RawLoopKind::DoWhile { condition } => {
                    self.infer_eager_expr(condition);
                }
            },
            RawStmtVariant::ConditionBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => {
                    self.infer_eager_expr(condition);
                }
                RawConditionBranchKind::Elif { condition } => {
                    self.infer_eager_expr(condition);
                }
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::Require { condition, .. } => {
                self.infer_eager_expr(condition);
            }
            RawStmtVariant::PatternBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { pattern } => self.infer_eager_case_pattern(pattern),
                RawPatternBranchVariant::Default => (),
            },
            RawStmtVariant::Exec {
                expr,
                discard: silent,
            } => {
                self.infer_eager_expr(expr);
            }
            RawStmtVariant::Init {
                init_kind,
                varname,
                initial_value,
            } => {
                if let Some(initial_value_qualified_ty) = self.infer_eager_expr(initial_value) {
                    self.qualified_ty_sheet
                        .eager_variable_qualified_tys
                        .insert_new((
                            (varname.ident.into(), varname.range),
                            initial_value_qualified_ty.init_variable_qualified_ty(init_kind),
                        ))
                        .unwrap()
                }
            }
            RawStmtVariant::Return { result, .. } => {
                match (opt_output_ty, self.infer_eager_expr(result)) {
                    (Some(output_ty), Some(qualified_ty)) => {
                        if !qualified_ty.is_implicitly_castable_to_output(
                            self.db,
                            output_liason,
                            output_ty,
                        ) {
                            self.qualified_ty_sheet.extra_errors.push(InferError {
                                variant: InferErrorVariant::Original {
                                    message: format!(
                                        "expect return type to be `{:?}`,\n  but got `{:?}` instead",
                                   output_ty,qualified_ty),
                                    range: stmt.range,
                                },
                                dev_src: husky_dev_utils::dev_src!(),
                            })
                        }
                    }
                    _ => (),
                }
            }
            RawStmtVariant::Assert(condition) => {
                self.infer_eager_expr(condition);
            }
            RawStmtVariant::Break => (),
            RawStmtVariant::Match { match_expr, .. } => {
                self.infer_eager_expr(match_expr);
            }
            RawStmtVariant::ReturnXml(ref xml_expr) => panic!(),
        }
    }

    fn infer_eager_case_pattern(&mut self, pattern: &RawPattern) {
        match pattern.variant {
            RawPatternVariant::PrimitiveLiteral(_) => (),
            RawPatternVariant::OneOf {
                subpatterns: ref patterns,
            } => (),
            RawPatternVariant::EnumLiteral(_) => (),
        }
    }

    pub(super) fn infer_eager_expr(&mut self, idx: RawExprIdx) -> Option<EagerValueQualifiedTy> {
        let qualified_qualified_ty_result = self.eager_expr(idx);
        let opt_qualified_ty = qualified_qualified_ty_result
            .as_ref()
            .map(|qualified_ty| *qualified_ty)
            .ok();
        self.qualified_ty_sheet
            .eager_expr_qualified_tys
            .insert_new(idx, qualified_qualified_ty_result);
        opt_qualified_ty
    }

    fn eager_expr(&mut self, raw_expr_idx: RawExprIdx) -> InferResult<EagerValueQualifiedTy> {
        let raw_expr = &self.arena[raw_expr_idx];
        match raw_expr.variant {
            RawExprVariant::Variable {
                varname,
                init_range,
            } => match derived_not_none!(self
                .qualified_ty_sheet
                .eager_variable_qualified_tys
                .get_entry((varname.into(), init_range)))?
            .1
            {
                Ok(variable_qt) => {
                    let variable_contract = self.eager_expr_contract(raw_expr_idx)?;
                    Ok(EagerValueQualifiedTy {
                        qual: variable_qt
                            .qual
                            .variable_use_eager_expr_qualifier(variable_contract, raw_expr.range)?,
                        ty: variable_qt.ty,
                    })
                }
                Err(ref e) => Err(e.derived()),
            },
            RawExprVariant::FrameVariable { .. } => Ok(EagerValueQualifiedTy {
                qual: EagerExprQualifier::Copyable,
                ty: EntityRoutePtr::Root(RootIdentifier::I32),
            }),
            RawExprVariant::ThisValue {
                opt_this_ty,
                opt_this_liason,
            } => {
                let this_ty = derived_not_none!(opt_this_ty)?;
                let this_liason = derived_not_none!(opt_this_liason)?;
                let this_contract = self.eager_expr_contract(raw_expr_idx)?;
                EagerValueQualifiedTy::from_parameter_use(
                    self.db,
                    this_liason,
                    this_ty,
                    this_contract,
                    raw_expr.range,
                )
            }
            RawExprVariant::ThisField {
                opt_this_ty,
                opt_this_liason,
                field_ident: ident,
                field_liason,
                opt_field_ty,
            } => {
                let this_ty = derived_not_none!(opt_this_ty)?;
                let this_liason = derived_not_none!(opt_this_liason)?;
                let field_contract = self.eager_expr_contract(raw_expr_idx)?;
                let field_ty = derived_not_none!(opt_field_ty)?;
                let is_field_copyable = self.db.is_copyable(field_ty.route)?;
                let this_contract = EagerContract::field_access_this_eager_contract(
                    field_liason,
                    field_contract,
                    is_field_copyable,
                    self.arena[raw_expr_idx].range,
                )?;
                let this_qual = EagerExprQualifier::parameter_use_eager_qualifier(
                    self.db.upcast(),
                    this_ty,
                    this_liason,
                    this_contract,
                    raw_expr.range,
                )?;
                Ok(EagerValueQualifiedTy::member_eager_qualified_ty(
                    self.db,
                    this_qual,
                    field_ty.route,
                    field_liason,
                    field_contract,
                    is_field_copyable,
                )?)
            }
            RawExprVariant::Unrecognized(_) => Err(derived!("unrecognized")),
            RawExprVariant::Entity { route, kind } => match kind {
                EntityKind::Module => Ok(EagerValueQualifiedTy::module_eager_qualified_ty()),
                EntityKind::Type(_) => Ok(EagerValueQualifiedTy::ty_eager_qualified_ty()),
                EntityKind::Trait => Ok(EagerValueQualifiedTy::trait_eager_qualified_ty()),
                EntityKind::Member(_) | EntityKind::Function { .. } => Ok(
                    EagerValueQualifiedTy::entity_ty(self.raw_expr_ty(raw_expr_idx)?),
                ),
                EntityKind::Feature => Ok(EagerValueQualifiedTy::feature_ty(
                    self.raw_expr_ty(raw_expr_idx)?,
                )),
                EntityKind::EnumVariant => Ok(EagerValueQualifiedTy {
                    qual: EagerExprQualifier::Copyable,
                    ty: self.raw_expr_intrinsic_ty(raw_expr_idx)?,
                }),
                EntityKind::Main => panic!(),
            },
            RawExprVariant::PrimitiveLiteral(_) => Ok(EagerValueQualifiedTy {
                qual: EagerExprQualifier::Copyable,
                ty: self.raw_expr_intrinsic_ty(raw_expr_idx)?,
            }),
            RawExprVariant::Bracketed(expr) => {
                derived_not_none!(self.infer_eager_expr(expr))
            }
            RawExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.eager_opn(raw_expr_idx, opn_variant, opds.clone(), raw_expr.range),
            RawExprVariant::Lambda(_, _) => todo!(),
        }
    }

    fn eager_opn(
        &mut self,
        idx: RawExprIdx,
        opr: &RawOpnVariant,
        opds: RawExprRange,
        range: TextRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        match opr {
            RawOpnVariant::Binary(binary_opr) => self.eager_binary(idx, *binary_opr, opds, range),
            RawOpnVariant::Prefix(prefix_opr) => self.eager_prefix(idx, opds),
            RawOpnVariant::Suffix(suffix_opr) => self.eager_suffix(idx, suffix_opr, opds),
            RawOpnVariant::List(list_opr) => self.eager_list(idx, list_opr, opds),
            RawOpnVariant::Field(field_ident) => self.eager_field_access(idx, *field_ident, opds),
        }
    }

    fn eager_binary(
        &mut self,
        idx: RawExprIdx,
        opr: BinaryOpr,
        opds: RawExprRange,
        range: TextRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(opds.start))?;
        match opr {
            BinaryOpr::Pure(_) => (),
            BinaryOpr::Assign(_) => match this_qt.qual {
                EagerExprQualifier::Copyable
                | EagerExprQualifier::PureRef
                | EagerExprQualifier::EvalRef
                | EagerExprQualifier::TempRef
                | EagerExprQualifier::Transient => throw!("lopd is not mutable", range),
                EagerExprQualifier::TempRefMut => (),
            },
        }
        self.infer_eager_expr(opds.start + 1);
        let ty = self.raw_expr_intrinsic_ty(idx)?;
        Ok(EagerValueQualifiedTy::new(
            if self.db.is_copyable(ty)? {
                EagerExprQualifier::Copyable
            } else {
                EagerExprQualifier::Transient
            },
            ty,
        ))
    }

    fn eager_prefix(
        &mut self,
        raw_expr_idx: RawExprIdx,
        opds: RawExprRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        self.infer_eager_expr(opds.start);
        let ty = self.raw_expr_intrinsic_ty(raw_expr_idx)?;
        Ok(EagerValueQualifiedTy::new(
            EagerExprQualifier::transitive(self.db.is_copyable(ty)?),
            self.raw_expr_intrinsic_ty(raw_expr_idx)?,
        ))
    }

    fn eager_suffix(
        &mut self,
        raw_expr_idx: RawExprIdx,
        opr: &RawSuffixOpr,
        opds: RawExprRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(opds.start))?;
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_qt.ty));
        match opr {
            RawSuffixOpr::Incr | RawSuffixOpr::Decr => Ok(EagerValueQualifiedTy {
                qual: EagerExprQualifier::Copyable,
                ty: EntityRoutePtr::Root(RootIdentifier::Void),
            }),
            RawSuffixOpr::AsTy(ranged_ty) => this_qt.as_ty(self.db, ranged_ty.route),
            RawSuffixOpr::BePattern(_) => todo!(),
        }
    }

    fn eager_field_access(
        &mut self,
        raw_expr_idx: RawExprIdx,
        field_ident: RangedCustomIdentifier,
        opds: RawExprRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(opds.start))?;
        let this_deref_ty = match this_qt.ty.variant {
            EntityRouteVariant::Root {
                ident: RootIdentifier::Ref,
            } => this_qt.ty.spatial_arguments[0].take_entity_route(),
            _ => this_qt.ty,
        };
        let this_ty_decl = derived_unwrap!(self.db.ty_decl(this_deref_ty));
        let field_decl = this_ty_decl.field_decl(field_ident)?;
        let field_contract = self.eager_expr_contract(raw_expr_idx)?;
        Ok(EagerValueQualifiedTy::member_eager_qualified_ty(
            self.db,
            this_qt.qual,
            field_decl.ty,
            field_decl.liason,
            field_contract,
            self.db.is_copyable(field_decl.ty)?,
        )?)
    }

    fn eager_list(
        &mut self,
        idx: RawExprIdx,
        list_opr: &ListOpr,
        opds: RawExprRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        match list_opr {
            ListOpr::TupleInit => todo!(),
            ListOpr::NewVec => self.eager_new_vec_from_list(idx, opds),
            ListOpr::NewDict => todo!(),
            ListOpr::FunctionCall => self.eager_function_call(idx, opds),
            ListOpr::Index | ListOpr::ModuloIndex => self.eager_element_access(idx, opds),
            ListOpr::StructInit => todo!(),
            ListOpr::MethodCall { ranged_ident, .. } => {
                self.eager_method_call(opds.start, *ranged_ident, (opds.start + 1)..opds.end, idx)
            }
        }
    }

    fn eager_new_vec_from_list(
        &mut self,
        idx: RawExprIdx,
        elements: RawExprRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        for element in elements {
            self.infer_eager_expr(element);
        }
        Ok(EagerValueQualifiedTy::new(
            EagerExprQualifier::Transient,
            self.raw_expr_ty(idx)?,
        ))
    }

    fn eager_function_call(
        &mut self,
        idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        let call_decl = self.function_call_form_decl(total_opds.start)?;
        self.infer_eager_expr(total_opds.start);
        for opd_idx in (total_opds.start + 1)..total_opds.end {
            let opd_contract = self.eager_expr_contract(opd_idx)?;
            if let Some(qt) = self.infer_eager_expr(opd_idx) {
                match (qt.qual, opd_contract) {
                    (EagerExprQualifier::Copyable, EagerContract::EvalRef) => {
                        panic!()
                    }
                    _ => (),
                }
            }
        }
        match call_decl.output.liason() {
            OutputLiason::Transfer => {
                msg_once!("handle ref");
                Ok(EagerValueQualifiedTy::new(
                    if self.db.is_copyable(call_decl.output.ty())? {
                        EagerExprQualifier::Copyable
                    } else {
                        EagerExprQualifier::Transient
                    },
                    call_decl.output.ty(),
                ))
            }
            OutputLiason::MemberAccess { .. } => todo!(),
        }
    }

    fn eager_element_access(
        &mut self,
        idx: RawExprIdx,
        total_opds: RawExprRange,
    ) -> InferResult<EagerValueQualifiedTy> {
        let this_qt = derived_not_none!(self.infer_eager_expr(total_opds.start))?;
        let this_contract = self.eager_expr_contract(total_opds.start)?;
        for opd in (total_opds.start + 1)..total_opds.end {
            self.infer_eager_expr(opd);
        }
        let element_ty = self.raw_expr_intrinsic_ty(idx)?;
        let element_contract = self.eager_expr_contract(idx)?;
        msg_once!("todo: other member liason");
        EagerValueQualifiedTy::member_eager_qualified_ty(
            self.db,
            this_qt.qual,
            element_ty,
            MemberLiason::Mutable,
            element_contract,
            self.db.is_copyable(element_ty)?,
        )
    }

    fn eager_method_call(
        &mut self,
        this: RawExprIdx,
        method_ident: RangedCustomIdentifier,
        inputs: RawExprRange,
        idx: RawExprIdx,
    ) -> InferResult<EagerValueQualifiedTy> {
        let call_form_decl = self.method_call_form_decl(this)?;
        let this_qt = derived_not_none!(self.infer_eager_expr(this))?;
        let this_contract = self.eager_expr_contract(this)?;
        for input in inputs {
            self.infer_eager_expr(input);
        }
        let is_element_copyable = self.db.is_copyable(call_form_decl.output.ty())?;
        let output_contract = self.eager_expr_contract(idx)?;
        let qual = match call_form_decl.output.liason() {
            OutputLiason::Transfer => {
                if is_element_copyable {
                    EagerExprQualifier::Copyable
                } else {
                    EagerExprQualifier::Transient
                }
            }
            OutputLiason::MemberAccess { member_liason } => EagerExprQualifier::member(
                this_qt.qual,
                member_liason,
                output_contract,
                is_element_copyable,
            ),
        };
        Ok(EagerValueQualifiedTy::new(qual, call_form_decl.output.ty()))
    }
}