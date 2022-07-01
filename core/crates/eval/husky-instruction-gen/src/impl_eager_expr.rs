use crate::*;
use entity_kind::TyKind;
use husky_ast::FieldAstKind;
use husky_linkage_table::ResolveLinkage;
use infer_decl::TyDecl;
use map_collect::MapCollect;
use vm::*;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_eager_expr(
        &mut self,
        expr: &Arc<EagerExpr>,
        output_stack_idx: VMStackIdx,
    ) {
        match expr.variant {
            EagerExprVariant::Variable { varname, binding } => {
                let stack_idx = self.sheet.variable_stack.stack_idx(varname);
                self.push_instruction(Instruction::new(
                    InstructionVariant::PushVariable {
                        varname: varname.into(),
                        stack_idx,
                        binding,
                        range: expr.range,
                        ty: expr.ty(),
                    },
                    expr.clone(),
                ))
            }
            EagerExprVariant::EntityRoute { route } => todo!(),
            EagerExprVariant::PrimitiveLiteral(value) => self.push_instruction(Instruction::new(
                InstructionVariant::PushPrimitiveLiteral {
                    value,
                    explicit: true,
                },
                expr.clone(),
            )),
            EagerExprVariant::Bracketed(ref expr) => {
                self.compile_eager_expr(expr, output_stack_idx)
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.compile_opn(opn_variant, opds, expr, output_stack_idx),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { binding } => self.push_instruction(Instruction::new(
                InstructionVariant::PushVariable {
                    varname: ContextualIdentifier::ThisValue.into(),
                    stack_idx: VMStackIdx::this(),
                    binding,
                    range: expr.range,
                    ty: expr.ty(),
                },
                expr.clone(),
            )),
            EagerExprVariant::ThisField {
                field_ident,
                field_idx,
                this_ty,
                this_binding,
                field_binding,
            } => match self.context.value() {
                InstructionGenContext::Normal => {
                    self.push_instruction(Instruction::new(
                        InstructionVariant::PushVariable {
                            varname: ContextualIdentifier::ThisValue.into(),
                            stack_idx: VMStackIdx::this(),
                            binding: this_binding,
                            range: expr.range,
                            ty: expr.ty(),
                        },
                        expr.clone(),
                    ));
                    self.push_instruction(Instruction::new(
                        if let Some(linkage) = self.db.compile_time().struct_field_access_linkage(
                            this_ty,
                            field_ident.ident,
                            field_binding,
                        ) {
                            InstructionVariant::CallSpecificRoutine { linkage }
                        } else {
                            let this_ty_decl = self.db.compile_time().ty_decl(this_ty).unwrap();
                            InstructionVariant::FieldAccessInterpreted {
                                field_idx: this_ty_decl
                                    .field_idx(field_ident.ident)
                                    .try_into()
                                    .unwrap(),
                                field_binding,
                            }
                        },
                        expr.clone(),
                    ))
                }
                InstructionGenContext::NewVirtualStruct { output_stack_idx } => self
                    .push_instruction(Instruction::new(
                        InstructionVariant::PushVariable {
                            varname: field_ident.ident.into(),
                            stack_idx: output_stack_idx + field_idx,
                            binding: field_binding,
                            range: expr.range,
                            ty: expr.ty(),
                        },
                        expr.clone(),
                    )),
            },
            EagerExprVariant::EnumKindLiteral(route) => self.push_instruction(Instruction::new(
                InstructionVariant::PushEnumKindLiteral(EnumKindValue {
                    kind_idx: self.db.enum_literal_as_u8(route),
                    route,
                }),
                expr.clone(),
            )),
        }
    }

    fn compile_opn(
        &mut self,
        opn_variant: &EagerOpnVariant,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
        output_stack_idx: VMStackIdx,
    ) {
        for (i, opd) in opds.iter().enumerate() {
            self.compile_eager_expr(opd, output_stack_idx + i);
        }
        match opn_variant {
            EagerOpnVariant::Binary { opr, this_ty } => {
                let ins_kind = InstructionVariant::OprOpn {
                    opn: match opr {
                        BinaryOpr::Pure(pure_binary_opr) => OprOpn::PureBinary(*pure_binary_opr),
                        BinaryOpr::Assign(opt_binary_opr) => OprOpn::BinaryAssign(*opt_binary_opr),
                    },
                    this_ty: *this_ty,
                    this_range: opds[0].range,
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EagerOpnVariant::Prefix { opr, this_ty } => {
                let instruction = Instruction::new(
                    InstructionVariant::OprOpn {
                        opn: OprOpn::Prefix(*opr),
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    expr.clone(),
                );
                self.push_instruction(instruction)
            }
            EagerOpnVariant::Suffix { opr, this_ty } => {
                let ins_kind = match opr {
                    SuffixOpr::Incr => InstructionVariant::OprOpn {
                        opn: OprOpn::Incr,
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    SuffixOpr::Decr => InstructionVariant::OprOpn {
                        opn: OprOpn::Decr,
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                    SuffixOpr::AsTy(ty) => InstructionVariant::OprOpn {
                        opn: match ty.route {
                            EntityRoutePtr::Root(ty_ident) => match ty_ident {
                                RootIdentifier::Void => todo!(),
                                RootIdentifier::I32 => OprOpn::Cast(CastOpn::AsI32),
                                RootIdentifier::F32 => OprOpn::Cast(CastOpn::AsF32),
                                RootIdentifier::B32 => OprOpn::Cast(CastOpn::AsB32),
                                RootIdentifier::B64 => todo!(),
                                RootIdentifier::Bool => todo!(),
                                _ => todo!(),
                            },
                            EntityRoutePtr::Custom(_) => todo!(),
                            EntityRoutePtr::ThisType => todo!(),
                        },
                        this_ty: *this_ty,
                        this_range: opds[0].range,
                    },
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EagerOpnVariant::RoutineCall(routine) => {
                if let Some(linkage) = self.db.compile_time().routine_linkage(routine.route) {
                    match linkage {
                        Linkage::MemberAccess { .. } => todo!(),
                        Linkage::SpecificTransfer(linkage) => {
                            self.push_instruction(Instruction::new(
                                InstructionVariant::CallSpecificRoutine { linkage },
                                expr.clone(),
                            ))
                        }
                        Linkage::GenericTransfer(_) => todo!(),
                        Linkage::Model(_) => todo!(),
                    }
                } else {
                    self.push_instruction(Instruction::new(
                        InstructionVariant::CallInterpreted {
                            routine_uid: self.db.compile_time().entity_uid(routine.route),
                            nargs: opds.len() as u8,
                            has_this: false,
                            output_ty: expr.ty(),
                        },
                        expr.clone(),
                    ))
                }
            }
            EagerOpnVariant::FieldAccess {
                this_ty,
                field_ident,
                field_liason,
                field_binding,
            } => {
                self.push_instruction(Instruction::new(
                    if let Some(linkage) = self.db.compile_time().struct_field_access_linkage(
                        *this_ty,
                        field_ident.ident,
                        *field_binding,
                    ) {
                        InstructionVariant::CallSpecificRoutine { linkage }
                    } else {
                        let this_ty_decl = self.db.compile_time().ty_decl(*this_ty).unwrap();
                        InstructionVariant::FieldAccessInterpreted {
                            field_idx: this_ty_decl
                                .field_idx(field_ident.ident)
                                .try_into()
                                .unwrap(),
                            field_binding: *field_binding,
                        }
                    },
                    expr.clone(),
                ));
            }
            EagerOpnVariant::MethodCall {
                method_ident,
                this_ty_decl,
                method_route,
                output_binding: opt_output_binding,
            } => {
                let this = &opds[0];
                self.push_instruction(Instruction::new(
                    self.method_call_instruction_variant(
                        this.ty(),
                        this_ty_decl,
                        *method_route,
                        method_ident.ident,
                        expr.ty(),
                        *opt_output_binding,
                    ),
                    expr.clone(),
                ))
            }
            EagerOpnVariant::Index { element_binding } => {
                self.compile_element_access(expr.clone(), opds, *element_binding)
            }
            EagerOpnVariant::TypeCall {
                ranged_ty,
                ref ty_decl,
            } => {
                let ty_defn = self.db.compile_time().entity_defn(ranged_ty.route).unwrap();
                let instruction_variant = match ty_defn.variant {
                    EntityDefnVariant::Ty {
                        kind,
                        ref ty_members,
                        ..
                    } => {
                        self.context.enter();
                        self.context
                            .set(InstructionGenContext::NewVirtualStruct { output_stack_idx });
                        for (i, ty_member) in ty_members.iter().enumerate() {
                            match ty_member.variant {
                                EntityDefnVariant::TyField {
                                    ty,
                                    ref field_variant,
                                    liason,
                                    ..
                                } => match field_variant {
                                    FieldDefnVariant::StructOriginal => (),
                                    FieldDefnVariant::StructDefault { default } => {
                                        msg_once!("handle keyword arguments");
                                        self.compile_eager_expr(default, output_stack_idx + i)
                                    }
                                    FieldDefnVariant::StructDerivedEager { derivation } => {
                                        self.compile_eager_expr(derivation, output_stack_idx + i)
                                    }
                                    FieldDefnVariant::StructDerivedLazy { .. } => break,
                                    FieldDefnVariant::RecordOriginal
                                    | FieldDefnVariant::RecordDerived { .. } => panic!(),
                                },
                                _ => break,
                            }
                        }
                        self.context.exit();
                        if let Some(linkage) =
                            self.db.compile_time().type_call_linkage(ranged_ty.route)
                        {
                            match linkage {
                                Linkage::SpecificTransfer(linkage) => {
                                    InstructionVariant::CallSpecificRoutine { linkage }
                                }
                                Linkage::GenericTransfer(linkage) => {
                                    InstructionVariant::CallGenericRoutine {
                                        output_ty: ranged_ty.route,
                                        linkage,
                                    }
                                }
                                Linkage::MemberAccess {
                                    copy_access,
                                    eval_ref_access,
                                    temp_ref_access,
                                    temp_mut_access,
                                    move_access,
                                } => todo!(),
                                Linkage::Model(_) => todo!(),
                            }
                        } else {
                            match kind {
                                TyKind::Enum => todo!(),
                                TyKind::Record => todo!(),
                                TyKind::Struct => InstructionVariant::NewVirtualStruct {
                                    ty: ranged_ty.route,
                                    fields: ty_decl.eager_fields().map(|decl| decl.ident).collect(),
                                },
                                TyKind::Primitive => todo!(),
                                TyKind::Vec => todo!(),
                                TyKind::Array => todo!(),
                                TyKind::Other => todo!(),
                            }
                        }
                    }
                    _ => panic!(),
                };
                self.push_instruction(Instruction::new(instruction_variant, expr.clone()))
            }
        }
    }

    fn compile_element_access(
        &mut self,
        this: Arc<EagerExpr>,
        opds: &[Arc<EagerExpr>],
        element_binding: Binding,
    ) {
        self.push_instruction(Instruction::new(
            InstructionVariant::CallSpecificRoutine {
                linkage: self
                    .db
                    .compile_time()
                    .element_access_linkage(opds.map(|opd| opd.ty()))
                    .bind(element_binding),
            },
            this,
        ))
    }

    fn method_call_instruction_variant(
        &self,
        this_ty: EntityRoutePtr,
        this_ty_decl: &TyDecl,
        method_route: EntityRoutePtr,
        method_ident: CustomIdentifier,
        output_ty: EntityRoutePtr,
        output_binding: Binding,
    ) -> InstructionVariant {
        if let Some(linkage) = self.db.compile_time().method_linkage(method_route) {
            match linkage {
                Linkage::MemberAccess { .. } => InstructionVariant::CallSpecificRoutine {
                    linkage: linkage.bind(output_binding),
                },
                Linkage::SpecificTransfer(linkage) => {
                    InstructionVariant::CallSpecificRoutine { linkage }
                }
                Linkage::GenericTransfer(linkage) => {
                    InstructionVariant::CallGenericRoutine { output_ty, linkage }
                }
                Linkage::Model(_) => todo!(),
            }
        } else {
            let method_uid = self.db.compile_time().entity_uid(method_route);
            let method_decl = self.db.compile_time().method_decl(method_route).unwrap();
            InstructionVariant::CallInterpreted {
                routine_uid: method_uid,
                nargs: (method_decl.parameters.len() + 1).try_into().unwrap(),
                has_this: true,
                output_ty,
            }
        }
    }
}