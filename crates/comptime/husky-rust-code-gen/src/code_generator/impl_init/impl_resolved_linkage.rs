use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_transfer_linkage(
        &mut self,
        needs_eval_context: bool,
        opt_this: Option<(ParameterLiason, EntityRoutePtr)>,
        gen_caller: impl FnOnce(&mut Self),
        gen_call_route: impl FnOnce(&mut Self),
        decl: &CallFormDecl,
    ) {
        let argidx_base = opt_this.map(|_| 1).unwrap_or(0);
        self.write(&format!(
            r#"
        transfer_linkage!(
            {{
                unsafe fn __wrapper<'eval>(
                    __opt_ctx: Option<&dyn __EvalContext<'eval>>,
                    __arguments: &mut [__Register<'eval>],
                ) -> __Register<'eval> {{ /*haha*/"#
        ));
        if let Some((this_liason, this_ty)) = opt_this {
            let mangled_this_ty_vtable = self.db.mangled_intrinsic_ty_vtable(this_ty);
            match this_liason {
                ParameterLiason::Pure => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&");
                        self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_temp_ref(&__registration__::{mangled_this_ty_vtable});"))
                    }
                }
                ParameterLiason::Move => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => panic!(),
                ParameterLiason::EvalRef => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    if self.db.is_copyable(this_ty).unwrap() {
                        todo!()
                    } else {
                        self.write("&'eval ");
                        self.gen_entity_route(this_ty.deref_route(), EntityRouteRole::Decl);
                        self.write(&format!(" = __arguments[0].downcast_eval_ref(&__registration__::{mangled_this_ty_vtable});"))
                    }
                }
                ParameterLiason::TempRef => todo!(),
                ParameterLiason::TempRefMut => {
                    self.write(&format!(
                        r#"
                    let __this: "#
                    ));
                    self.write("&mut ");
                    self.gen_entity_route(this_ty, EntityRouteRole::Decl);
                    self.write(&format!(
                        " = unsafe {{ __arb_ref(&__arguments[0]) }}.downcast_temp_mut(&__registration__::{mangled_this_ty_vtable});"
                    ))
                }
            }
        }
        for (i, parameter) in decl.primary_parameters.iter().enumerate() {
            self.gen_parameter_downcast(i + argidx_base, parameter)
        }
        msg_once!("keyword parameter overrides");
        for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
            let parameter_name = parameter.ident;
            let parameter_ty = parameter.ty;
            self.write(&format!(
                r#"
                    let {parameter_name}: "#
            ));
            self.gen_entity_route(parameter_ty, EntityRouteRole::Decl);
            self.write(&format!(" = todo!();"))
        }
        self.gen_variadics_downcast(decl);
        self.write(&format!(
            r#"
                    "#
        ));
        let output_ty = decl.output.ty();
        let canonical_output_ty = output_ty.canonicalize();
        let output_ty_reg_memory_kind = self.db.reg_memory_kind(output_ty);
        let is_intrinsic_output_ty_primitive = canonical_output_ty.is_intrinsic_route_primitive();
        match canonical_output_ty.kind() {
            CanonicalEntityRoutePtrKind::Intrinsic => match output_ty_reg_memory_kind {
                RegMemoryKind::Direct => {
                    if is_intrinsic_output_ty_primitive {
                        // pass
                        ()
                    } else {
                        todo!()
                    }
                }
                RegMemoryKind::BoxCopyable | RegMemoryKind::BoxNonCopyable => {
                    self.write("__Register::new_box::<");
                    self.gen_entity_route(
                        canonical_output_ty.intrinsic_route(),
                        EntityRouteRole::Decl,
                    );
                    self.write(">(");
                }
            },
            CanonicalEntityRoutePtrKind::Optional => match output_ty_reg_memory_kind {
                RegMemoryKind::Direct => {
                    if is_intrinsic_output_ty_primitive {
                        // pass
                        ()
                    } else {
                        todo!()
                    }
                }
                RegMemoryKind::BoxCopyable => todo!(),
                RegMemoryKind::BoxNonCopyable => todo!(),
            },
            CanonicalEntityRoutePtrKind::EvalRef => todo!(),
            CanonicalEntityRoutePtrKind::OptionalEvalRef => {
                self.write("__Register::new_opt_eval_ref::<");
                self.gen_entity_route(canonical_output_ty.intrinsic_route(), EntityRouteRole::Decl);
                self.write(">(");
            }
        }
        gen_caller(self);
        self.write("(");
        if needs_eval_context {
            self.write("__opt_ctx.unwrap()")
        }
        for (i, parameter) in decl.primary_parameters.iter().enumerate() {
            if needs_eval_context || i > 0 {
                self.write(", ")
            }
            self.write(&parameter.ident)
        }
        for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
            if needs_eval_context || i + decl.primary_parameters.len() > 0 {
                self.write(", ");
            }
            self.write(&parameter.ident)
        }
        match decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { .. } => {
                if needs_eval_context
                    || decl.primary_parameters.len() > 0
                    || decl.keyword_parameters.len() > 0
                {
                    self.write(", ")
                }
                self.write("__variadics")
            }
        }
        let mangled_output_ty_vtable = self.db.mangled_intrinsic_ty_vtable(decl.output.ty());
        if is_intrinsic_output_ty_primitive {
            self.write(&format!(
                r#").to_register()
                }}
                __wrapper
            }},
            some "#
            ));
        } else {
            self.write(&format!(
                r#"), &__registration__::{mangled_output_ty_vtable})
                }}
                __wrapper
            }},
            some "#
            ));
        }
        gen_call_route(self);
        self.write(r#" as "#);
        self.gen_call_ty(needs_eval_context, decl);
        self.write(
            r#"
        ),"#,
        )
    }

    fn gen_variadics_downcast(&mut self, decl: &CallFormDecl) {
        match decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { variadic_ty } => {
                let variadic_start = decl.variadic_start();
                if variadic_ty.is_primitive() {
                    self.write(&format!(
                        r#"
                    let __variadics =
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v|v.downcast_{variadic_ty}())
                            .collect();"#,
                    ));
                } else {
                    let variadic_ty_vtable = self.db.mangled_intrinsic_ty_vtable(variadic_ty);
                    match self.db.is_copyable(variadic_ty).unwrap() {
                        true => {
                            if variadic_ty.is_option() {
                                let variadic_ty = variadic_ty.entity_route_argument(0);
                                if variadic_ty.is_ref() {
                                    self.write(&format!(
                                    r#"
                    let __variadics =
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v|v.downcast_opt_eval_ref(&__registration__::{variadic_ty_vtable}))
                            .collect();"#,
                    ));
                                } else if variadic_ty.is_primitive() {
                                    self.write(&format!(
                                        r#"
                    let __variadics =
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v|v.downcast_opt_{}())
                            .collect();"#,
                                        variadic_ty.ident().as_str()
                                    ));
                                } else {
                                    todo!()
                                }
                            } else if variadic_ty.is_ref() {
                                self.write(&format!(
                                    r#"
                    let __variadics =
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v|v.downcast_eval_ref(&__registration__::{variadic_ty_vtable}))
                            .collect();"#,
                                ));
                            } else if variadic_ty.is_fp() {
                                self.write(&format!(
                                    r#"
                    let __variadics =
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v| {{
                                std::mem::transmute(v.downcast_temp_ref::<__VirtualFunction>(&__registration__::{variadic_ty_vtable}).fp())
                            }})
                            .collect();"#,
                                    ));
                            } else {
                                p!(variadic_ty);
                                todo!()
                            }
                        }
                        false => {
                            if variadic_ty.is_option() {
                                todo!()
                            } else {
                                self.write(&format!(
                                    r#"
                    let __variadics =
                        __arguments[{variadic_start}..]
                            .iter_mut()
                            .map(|v|v.downcast_move(&__registration__::{variadic_ty_vtable}))
                            .collect();"#,
                                ));
                            }
                        }
                    };
                }
            }
        }
    }

    fn gen_call_ty(&mut self, needs_eval_context: bool, decl: &CallFormDecl) {
        self.write("fn(");
        if needs_eval_context {
            self.write("&__EvalContext<'static>, ")
        }
        if let Some(this_ty) = decl.opt_this_ty() {
            match decl.opt_this_liason.unwrap() {
                ParameterLiason::Pure => {
                    if self.db.is_copyable(this_ty).unwrap() {
                        ()
                    } else {
                        self.write("&'static ")
                    }
                }
                ParameterLiason::Move => todo!(),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::EvalRef => todo!(),
                ParameterLiason::TempRef => todo!(),
                ParameterLiason::TempRefMut => self.write("&'static mut "),
            }
            self.gen_entity_route(this_ty, EntityRouteRole::StaticDecl)
        }
        for (i, parameter) in decl.primary_parameters.iter().enumerate() {
            if needs_eval_context || decl.opt_this_liason.is_some() || i > 0 {
                self.write(", ")
            }
            match parameter.liason {
                ParameterLiason::Pure => {
                    if self.db.is_copyable(parameter.ty).unwrap() {
                        ()
                    } else {
                        self.write("&'static ")
                    }
                }
                ParameterLiason::Move => (),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::EvalRef => self.write("&'static "),
                ParameterLiason::TempRef => todo!(),
                ParameterLiason::TempRefMut => todo!(),
            }
            self.gen_entity_route(parameter.ty, EntityRouteRole::StaticDecl)
        }
        for (i, parameter) in decl.keyword_parameters.iter().enumerate() {
            if needs_eval_context
                || decl.opt_this_liason.is_some()
                || i + decl.primary_parameters.len() > 0
            {
                self.write(", ");
            }
            match parameter.liason {
                ParameterLiason::Pure => {
                    if self.db.is_copyable(parameter.ty).unwrap() {
                        ()
                    } else {
                        self.write("&'static ")
                    }
                }
                ParameterLiason::Move => (),
                ParameterLiason::MoveMut => todo!(),
                ParameterLiason::MemberAccess => todo!(),
                ParameterLiason::EvalRef => self.write("&'static "),
                ParameterLiason::TempRef => todo!(),
                ParameterLiason::TempRefMut => todo!(),
            }
            self.gen_entity_route(parameter.ty, EntityRouteRole::StaticDecl)
        }
        match decl.variadic_template {
            VariadicTemplate::None => (),
            VariadicTemplate::SingleTyped { variadic_ty } => {
                if needs_eval_context
                    || decl.opt_this_liason.is_some()
                    || decl.primary_parameters.len() > 0
                    || decl.keyword_parameters.len() > 0
                {
                    self.write(", ")
                }
                self.write("Vec<");
                self.gen_entity_route(variadic_ty, EntityRouteRole::StaticDecl);
                self.write(">")
            }
        }
        self.write(") -> ");
        self.gen_entity_route(decl.output.ty(), EntityRouteRole::StaticDecl)
    }
}