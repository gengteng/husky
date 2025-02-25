use crate::*;
use husky_entity_kind::{FieldKind, TyKind};
use husky_linkage_table::ResolveLinkage;
use husky_opn_semantics::{EagerSuffixOpr, ImplicitConversion};
use husky_primitive_literal_semantics::convert_primitive_literal_to_register;
use husky_vm::{
    __root::{__ASSIGN_LINKAGE, __EQ_LINKAGE, __NEQ_LINKAGE, __VALUE_CALL_LINKAGE},
    *,
};
use husky_vm_primitive_opr_linkage::{
    resolve_primitive_assign_binary_opr_linkage, resolve_primitive_prefix_opr_linkage,
    resolve_primitive_pure_binary_opr_linkage, resolve_primitive_suffix_opr_linkage,
};
use infer_decl::TyDecl;
use map_collect::MapCollect;

impl<'a> InstructionSheetBuilder<'a> {
    pub(super) fn compile_eager_expr(
        &mut self,
        expr: &Arc<EagerExpr>,
        output_stack_idx: VMStackIdx,
        discard: bool,
    ) {
        match expr.variant {
            EagerExprVariant::Variable { varname, binding } => {
                // no discard
                assert!(!discard);
                let stack_idx = self.sheet.variable_stack.stack_idx(varname);
                self.push_instruction(Instruction::new(
                    InstructionVariant::PushVariable {
                        varname: varname.into(),
                        stack_idx,
                        binding,
                        range: expr.range,
                        ty: expr.intrinsic_ty(),
                        explicit: true,
                    },
                    expr.clone(),
                ))
            }
            EagerExprVariant::PrimitiveLiteral(value) => {
                // no discard
                assert!(!discard);
                self.push_instruction(Instruction::new(
                    InstructionVariant::PushLiteralValue {
                        value: convert_primitive_literal_to_register(value, expr.intrinsic_ty()),
                        explicit: true,
                        ty: expr.intrinsic_ty(),
                    },
                    expr.clone(),
                ))
            }
            EagerExprVariant::Bracketed(ref expr) => {
                self.compile_eager_expr(expr, output_stack_idx, discard)
            }
            EagerExprVariant::Opn {
                ref opn_variant,
                ref opds,
            } => self.compile_opn(opn_variant, opds, expr, output_stack_idx, discard),
            EagerExprVariant::Lambda(_, _) => todo!(),
            EagerExprVariant::ThisValue { binding } => self.push_instruction(Instruction::new(
                InstructionVariant::PushVariable {
                    varname: ContextualIdentifier::ThisValue.into(),
                    stack_idx: VMStackIdx::this(),
                    binding,
                    range: expr.range,
                    ty: expr.intrinsic_ty(),
                    explicit: true,
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
                            ty: this_ty,
                            explicit: false,
                        },
                        expr.clone(),
                    ));
                    self.push_instruction(Instruction::new(
                        if let Some(linkage) = self.db.comptime().field_linkage_resolved(
                            this_ty.intrinsic(),
                            field_ident.ident,
                            field_binding,
                        ) {
                            InstructionVariant::CallRoutine {
                                output_ty: expr.intrinsic_ty(),
                                nargs: 1,
                                resolved_linkage: linkage,
                                discard,
                            }
                        } else {
                            let this_ty_decl = self.db.comptime().ty_decl(this_ty).unwrap();
                            let field_decl = this_ty_decl.field_decl(field_ident).unwrap();
                            match field_decl.field_kind {
                                FieldKind::StructRegular
                                | FieldKind::StructDefault
                                | FieldKind::StructDerived => {
                                    InstructionVariant::VirtualStructField {
                                        field_idx: this_ty_decl
                                            .field_idx(field_ident.ident)
                                            .try_into()
                                            .unwrap(),
                                        field_binding,
                                        field_ty: expr.intrinsic_ty(),
                                    }
                                }
                                FieldKind::StructMemo => todo!(),
                                FieldKind::RecordRegular => todo!(),
                                FieldKind::RecordProperty => todo!(),
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
                            ty: expr.intrinsic_ty(),
                            explicit: true,
                        },
                        expr.clone(),
                    )),
            },
            EagerExprVariant::EnumKindLiteral(route) => self.push_instruction(Instruction::new(
                InstructionVariant::PushLiteralValue {
                    value: __VirtualEnum {
                        kind_idx: self.db.enum_literal_to_i32(route),
                    }
                    .to_register(),
                    ty: expr.intrinsic_ty(),
                    explicit: true,
                },
                expr.clone(),
            )),
            EagerExprVariant::EntityFeature { route } => self.push_instruction(Instruction::new(
                InstructionVariant::EntityFeature {
                    feature_uid: self.db.comptime().entity_uid(route),
                    ty: expr.intrinsic_ty(),
                },
                expr.clone(),
            )),
            EagerExprVariant::EntityThickFp { route } => self.push_instruction(Instruction::new(
                InstructionVariant::PushEntityFp {
                    opt_linkage: self.db.comptime().routine_linkage(route),
                    opt_instruction_sheet: self.db.entity_instruction_sheet(route),
                    ty: expr.intrinsic_ty(),
                },
                expr.clone(),
            )),
        }
        match expr.implicit_conversion {
            ImplicitConversion::None => (),
            ImplicitConversion::WrapInSome { number_of_somes } => {
                self.push_instruction(Instruction::new(
                    InstructionVariant::WrapInSome { number_of_somes },
                    expr.clone(),
                ))
            }
            ImplicitConversion::ConvertToBool => todo!(),
        }
    }

    fn compile_opn(
        &mut self,
        opn_variant: &EagerOpnVariant,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
        output_stack_idx: VMStackIdx,
        discard: bool,
    ) {
        for (i, opd) in opds.iter().enumerate() {
            self.compile_eager_expr(opd, output_stack_idx + i, false);
        }
        match opn_variant {
            EagerOpnVariant::Binary { opr } => self.compile_binary_opn(*opr, opds, expr, discard),
            EagerOpnVariant::Prefix { opr } => self.compile_prefix_opn(*opr, opds, expr, discard),
            EagerOpnVariant::Suffix { ref opr, .. } => {
                self.compile_suffix(opr, opds, expr, discard)
            }
            EagerOpnVariant::RoutineCall(routine) => {
                if let Some(linkage) = self.db.comptime().routine_linkage(routine.route) {
                    match linkage {
                        __Linkage::Member { .. } => todo!(),
                        __Linkage::Transfer(resolved_linkage) => {
                            self.push_instruction(Instruction::new(
                                InstructionVariant::CallRoutine {
                                    output_ty: expr.intrinsic_ty(),
                                    nargs: opds.len().try_into().unwrap(),
                                    resolved_linkage,
                                    discard,
                                },
                                expr.clone(),
                            ))
                        }
                        __Linkage::Model(_) => todo!(),
                    }
                } else {
                    self.push_instruction(Instruction::new(
                        InstructionVariant::CallInterpreted {
                            routine_uid: self.db.comptime().entity_uid(routine.route),
                            nargs: opds.len().try_into().unwrap(),
                            output_ty: expr.intrinsic_ty(),
                            discard,
                        },
                        expr.clone(),
                    ))
                }
            }
            EagerOpnVariant::Field {
                this_ty,
                field_ident,
                field_binding,
                ..
            } => {
                // no discard
                assert!(!discard);
                self.push_instruction(Instruction::new(
                    if let Some(linkage) = self.db.comptime().field_linkage_resolved(
                        *this_ty,
                        field_ident.ident,
                        *field_binding,
                    ) {
                        InstructionVariant::CallRoutine {
                            resolved_linkage: linkage,
                            nargs: 1,
                            output_ty: expr.intrinsic_ty(),
                            discard,
                        }
                    } else {
                        let this_ty_decl = self.db.comptime().ty_decl(*this_ty).unwrap();
                        let field_decl = this_ty_decl.field_decl(*field_ident).unwrap();
                        match field_decl.field_kind {
                            FieldKind::StructRegular
                            | FieldKind::StructDefault
                            | FieldKind::StructDerived => InstructionVariant::VirtualStructField {
                                field_idx: this_ty_decl
                                    .field_idx(field_ident.ident)
                                    .try_into()
                                    .unwrap(),
                                field_binding: *field_binding,
                                field_ty: expr.intrinsic_ty(),
                            },
                            FieldKind::StructMemo => todo!(),
                            FieldKind::RecordRegular => todo!(),
                            FieldKind::RecordProperty => todo!(),
                        }
                    },
                    expr.clone(),
                ));
            }
            EagerOpnVariant::MethodCall {
                method_route,
                output_binding,
                ..
            } => self.push_instruction(Instruction::new(
                self.method_call_instruction_variant(
                    *method_route,
                    expr.intrinsic_ty(),
                    *output_binding,
                    opds.len().try_into().unwrap(),
                    discard,
                ),
                expr.clone(),
            )),
            EagerOpnVariant::Index { element_binding } => {
                // no discard
                assert!(!discard);
                self.compile_element_access(expr.clone(), opds, *element_binding)
            }
            EagerOpnVariant::TypeCall {
                ranged_ty,
                ref ty_decl,
            } => {
                // no discard
                assert!(!discard);
                let ty_defn = self.db.comptime().entity_defn(ranged_ty.route).unwrap();
                let instruction_variant = match ty_defn.variant {
                    EntityDefnVariant::Ty {
                        ty_kind: kind,
                        ref ty_members,
                        ..
                    } => {
                        self.context.enter();
                        self.context
                            .set(InstructionGenContext::NewVirtualStruct { output_stack_idx });
                        for (i, ty_member) in ty_members.iter().enumerate() {
                            match ty_member.variant {
                                EntityDefnVariant::TyField {
                                    ref field_variant, ..
                                } => match field_variant {
                                    FieldDefnVariant::StructOriginal => (),
                                    FieldDefnVariant::StructDefault { default } => {
                                        msg_once!("handle keyword arguments");
                                        self.compile_eager_expr(
                                            default,
                                            output_stack_idx + i,
                                            false,
                                        )
                                    }
                                    FieldDefnVariant::StructDerivedEager { derivation } => self
                                        .compile_eager_expr(
                                            derivation,
                                            output_stack_idx + i,
                                            false,
                                        ),
                                    FieldDefnVariant::StructDerivedLazy { .. } => break,
                                    FieldDefnVariant::RecordOriginal
                                    | FieldDefnVariant::RecordDerived { .. } => panic!(),
                                },
                                _ => break,
                            }
                        }
                        self.context.exit();
                        if let Some(linkage) = self.db.comptime().type_call_linkage(ranged_ty.route)
                        {
                            match linkage {
                                __Linkage::Transfer(linkage) => InstructionVariant::CallRoutine {
                                    output_ty: expr.intrinsic_ty(),
                                    nargs: opds.len().try_into().unwrap(),
                                    resolved_linkage: linkage,
                                    discard,
                                },
                                __Linkage::Member(_) => todo!(),
                                __Linkage::Model(_) => todo!(),
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
                                TyKind::Slice => todo!(),
                                TyKind::CyclicSlice => todo!(),
                                TyKind::Tuple => todo!(),
                                TyKind::Mor => todo!(),
                                TyKind::ThickFp => todo!(),
                                TyKind::AssociatedAny => todo!(),
                                TyKind::ThisAny => todo!(),
                                TyKind::SpatialPlaceholderAny => todo!(),
                                TyKind::BoxAny => todo!(),
                                TyKind::HigherKind => todo!(),
                                TyKind::Ref => todo!(),
                                TyKind::Option => todo!(),
                                TyKind::TargetOutputAny => todo!(),
                            }
                        }
                    }
                    _ => panic!(),
                };
                self.push_instruction(Instruction::new(instruction_variant, expr.clone()))
            }
            EagerOpnVariant::NewVecFromList => {
                // no discard
                assert!(!discard);
                let output_ty = expr.intrinsic_ty();
                let linkage = self.db.comptime().type_call_linkage(output_ty).unwrap();
                self.push_instruction(Instruction::new(
                    match linkage {
                        __Linkage::Member(_) => todo!(),
                        __Linkage::Transfer(linkage) => InstructionVariant::CallRoutine {
                            resolved_linkage: linkage,
                            nargs: opds.len().try_into().unwrap(),
                            output_ty,
                            discard,
                        },
                        __Linkage::Model(_) => todo!(),
                    },
                    expr.clone(),
                ))
            }
            EagerOpnVariant::ValueCall => self.push_instruction(Instruction::new(
                InstructionVariant::CallRoutine {
                    resolved_linkage: __VALUE_CALL_LINKAGE.transfer(),
                    nargs: opds.len().try_into().unwrap(),
                    output_ty: expr.intrinsic_ty(),
                    discard,
                },
                expr.clone(),
            )),
        }
    }

    fn compile_suffix(
        &mut self,
        opr: &EagerSuffixOpr,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
        discard: bool,
    ) {
        let this_ty = opds[0].intrinsic_ty();
        let ins_variant = InstructionVariant::CallRoutine {
            resolved_linkage: match opr {
                EagerSuffixOpr::Incr | EagerSuffixOpr::Decr | EagerSuffixOpr::AsTy(_) => {
                    match this_ty {
                        EntityRoutePtr::Root(root_identifier) => {
                            resolve_primitive_suffix_opr_linkage(opr, root_identifier).transfer()
                        }
                        EntityRoutePtr::Custom(_) => {
                            let ty_decl: Arc<TyDecl> = self.db.comptime().ty_decl(this_ty).unwrap();
                            match ty_decl.ty_kind {
                                TyKind::Enum => match opr {
                                    EagerSuffixOpr::Incr => todo!(),
                                    EagerSuffixOpr::Decr => todo!(),
                                    EagerSuffixOpr::AsTy(as_ty) => match as_ty.route {
                                        EntityRoutePtr::Root(root_identifier) => {
                                            match root_identifier {
                                                RootIdentifier::Void => todo!(),
                                                RootIdentifier::I32 => transfer_linkage!(
                                                    |args, _| {
                                                        let enum_value: &__VirtualEnum = args[0]
                                                            .downcast_temp_ref(
                                                                &__VIRTUAL_ENUM_VTABLE,
                                                            );
                                                        enum_value.kind_idx.to_register()
                                                    },
                                                    none
                                                )
                                                .transfer(),
                                                RootIdentifier::I64 => todo!(),
                                                RootIdentifier::F32 => todo!(),
                                                RootIdentifier::F64 => todo!(),
                                                RootIdentifier::B32 => todo!(),
                                                RootIdentifier::B64 => todo!(),
                                                RootIdentifier::Bool => todo!(),
                                                RootIdentifier::True => todo!(),
                                                RootIdentifier::False => todo!(),
                                                RootIdentifier::Vec => todo!(),
                                                RootIdentifier::Tuple => todo!(),
                                                RootIdentifier::Debug => todo!(),
                                                RootIdentifier::Std => todo!(),
                                                RootIdentifier::Core => todo!(),
                                                RootIdentifier::Mor => todo!(),
                                                RootIdentifier::ThickFp => todo!(),
                                                RootIdentifier::Fn => todo!(),
                                                RootIdentifier::FnMut => todo!(),
                                                RootIdentifier::FnOnce => todo!(),
                                                RootIdentifier::Array => todo!(),
                                                RootIdentifier::Domains => todo!(),
                                                RootIdentifier::DatasetType => todo!(),
                                                RootIdentifier::VisualType => todo!(),
                                                RootIdentifier::TypeType => todo!(),
                                                RootIdentifier::TraitType => todo!(),
                                                RootIdentifier::ModuleType => todo!(),
                                                RootIdentifier::CloneTrait => todo!(),
                                                RootIdentifier::CopyTrait => todo!(),
                                                RootIdentifier::PartialEqTrait => todo!(),
                                                RootIdentifier::EqTrait => todo!(),
                                                RootIdentifier::Ref => todo!(),
                                                RootIdentifier::RefMut => todo!(),
                                                RootIdentifier::Option => todo!(),
                                            }
                                        }
                                        EntityRoutePtr::Custom(_) => todo!(),
                                    },
                                    EagerSuffixOpr::BePattern(_) => todo!(),
                                    EagerSuffixOpr::Unveil => todo!(),
                                },
                                TyKind::Record => todo!(),
                                TyKind::Struct => todo!(),
                                TyKind::Primitive => todo!(),
                                TyKind::Vec => todo!(),
                                TyKind::Array => todo!(),
                                TyKind::Slice => todo!(),
                                TyKind::CyclicSlice => todo!(),
                                TyKind::Tuple => todo!(),
                                TyKind::Mor => todo!(),
                                TyKind::ThickFp => todo!(),
                                TyKind::AssociatedAny => todo!(),
                                TyKind::ThisAny => todo!(),
                                TyKind::SpatialPlaceholderAny => todo!(),
                                TyKind::BoxAny => todo!(),
                                TyKind::HigherKind => todo!(),
                                TyKind::Ref => todo!(),
                                TyKind::Option => todo!(),
                                TyKind::TargetOutputAny => todo!(),
                            }
                        }
                    }
                }
                EagerSuffixOpr::BePattern(_) => todo!(),
                EagerSuffixOpr::Unveil => todo!(),
            },
            nargs: 1,
            output_ty: expr.intrinsic_ty(),
            discard,
        };
        let instruction = Instruction::new(ins_variant, expr.clone());
        self.push_instruction(instruction)
    }

    fn compile_binary_opn(
        &mut self,
        opr: BinaryOpr,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
        discard: bool,
    ) {
        match opds[0].intrinsic_ty() {
            EntityRoutePtr::Root(_) => {
                let ins_kind = InstructionVariant::CallRoutine {
                    resolved_linkage: match opr {
                        BinaryOpr::Pure(pure_binary_opr) => {
                            resolve_primitive_pure_binary_opr_linkage(
                                opds[0].intrinsic_ty().root(),
                                pure_binary_opr,
                                opds[1].intrinsic_ty().root(),
                            )
                            .transfer()
                        }
                        BinaryOpr::Assign(opt_binary_opr) => {
                            resolve_primitive_assign_binary_opr_linkage(
                                opds[0].intrinsic_ty().root(),
                                opt_binary_opr,
                                opds[1].intrinsic_ty().root(),
                            )
                            .transfer()
                        }
                    },
                    nargs: 2,
                    output_ty: expr.intrinsic_ty(),
                    discard,
                };
                let instruction = Instruction::new(ins_kind, expr.clone());
                self.push_instruction(instruction)
            }
            EntityRoutePtr::Custom(_) => {
                let ins_variant = match opr {
                    BinaryOpr::Pure(pure_binary_opr) => match pure_binary_opr {
                        PureBinaryOpr::Add => todo!(),
                        PureBinaryOpr::And => todo!(),
                        PureBinaryOpr::BitAnd => todo!(),
                        PureBinaryOpr::BitOr => todo!(),
                        PureBinaryOpr::BitXor => todo!(),
                        PureBinaryOpr::Div => todo!(),
                        PureBinaryOpr::Eq => InstructionVariant::CallRoutine {
                            resolved_linkage: __EQ_LINKAGE.transfer(),
                            nargs: 2,
                            output_ty: expr.intrinsic_ty(),
                            discard,
                        },
                        PureBinaryOpr::Geq => todo!(),
                        PureBinaryOpr::Greater => todo!(),
                        PureBinaryOpr::Leq => todo!(),
                        PureBinaryOpr::Less => todo!(),
                        PureBinaryOpr::Mul => todo!(),
                        PureBinaryOpr::Neq => InstructionVariant::CallRoutine {
                            resolved_linkage: __NEQ_LINKAGE.transfer(),
                            nargs: 2,
                            output_ty: expr.intrinsic_ty(),
                            discard,
                        },
                        PureBinaryOpr::RemEuclid => todo!(),
                        PureBinaryOpr::Or => todo!(),
                        PureBinaryOpr::Power => todo!(),
                        PureBinaryOpr::Shl => todo!(),
                        PureBinaryOpr::Shr => todo!(),
                        PureBinaryOpr::Sub => todo!(),
                    },
                    BinaryOpr::Assign(opt_binary_opr) => {
                        if let Some(binary_opr) = opt_binary_opr {
                            todo!()
                        } else {
                            InstructionVariant::CallRoutine {
                                resolved_linkage: __ASSIGN_LINKAGE.transfer(),
                                nargs: 2,
                                output_ty: RootIdentifier::Void.into(),
                                discard,
                            }
                        }
                    }
                };
                let instruction = Instruction::new(ins_variant, expr.clone());
                self.push_instruction(instruction)
            }
        }
    }

    fn compile_prefix_opn(
        &mut self,
        prefix: PrefixOpr,
        opds: &[Arc<EagerExpr>],
        expr: &Arc<EagerExpr>,
        discard: bool,
    ) {
        let this = &opds[0];
        let this_ty = this.intrinsic_ty();
        match this_ty {
            EntityRoutePtr::Root(_) => {
                let instruction = Instruction::new(
                    InstructionVariant::CallRoutine {
                        resolved_linkage: resolve_primitive_prefix_opr_linkage(
                            prefix,
                            opds[0].intrinsic_ty().root(),
                        )
                        .transfer(),
                        nargs: 1,
                        output_ty: expr.intrinsic_ty(),
                        discard,
                    },
                    expr.clone(),
                );
                self.push_instruction(instruction)
            }
            EntityRoutePtr::Custom(_) => todo!(),
        }
    }

    fn compile_element_access(
        &mut self,
        expr: Arc<EagerExpr>,
        opds: &[Arc<EagerExpr>],
        element_binding: Binding,
    ) {
        self.push_instruction(Instruction::new(
            InstructionVariant::CallRoutine {
                output_ty: expr.intrinsic_ty(),
                nargs: opds.len().try_into().unwrap(),
                resolved_linkage: self
                    .db
                    .comptime()
                    .index_linkage(opds.map(|opd| opd.intrinsic_ty()))
                    .bind(element_binding),
                discard: false,
            },
            expr,
        ))
    }

    fn method_call_instruction_variant(
        &self,
        method_route: EntityRoutePtr,
        output_ty: EntityRoutePtr,
        output_binding: Binding,
        nargs: u8,
        discard: bool,
    ) -> InstructionVariant {
        if let Some(linkage) = self.db.comptime().method_linkage(method_route) {
            match linkage {
                __Linkage::Member { .. } => InstructionVariant::CallRoutine {
                    resolved_linkage: linkage.bind(output_binding),
                    nargs,
                    output_ty,
                    discard,
                },
                __Linkage::Transfer(linkage) => InstructionVariant::CallRoutine {
                    output_ty,
                    nargs,

                    resolved_linkage: linkage,
                    discard,
                },
                __Linkage::Model(_) => todo!(),
            }
        } else {
            let method_uid = self.db.comptime().entity_uid(method_route);
            let call_form_decl = self
                .db
                .comptime()
                .entity_call_form_decl(method_route)
                .unwrap();
            InstructionVariant::CallInterpreted {
                routine_uid: method_uid,
                nargs: (call_form_decl.primary_parameters.len() + 1)
                    .try_into()
                    .unwrap(),
                output_ty,
                discard,
            }
        }
    }
}
