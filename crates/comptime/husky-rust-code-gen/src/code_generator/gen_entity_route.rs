use husky_entity_route::*;
use husky_word::RootIdentifier;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_entity_route(&mut self, entity_route: EntityRoutePtr, role: EntityRouteRole) {
        if let Some(_) = self
            .entity_route_uses
            .find(|candidate| candidate.variant == entity_route.variant)
        {
            self.write(&entity_route.ident())
        } else {
            match entity_route.variant {
                EntityRouteVariant::Root { ident } => match ident {
                    RootIdentifier::Void => self.write("()"),
                    RootIdentifier::B32 => self.write("u32"),
                    RootIdentifier::B64 => self.write("u64"),
                    RootIdentifier::Std => self.write("__std"),
                    RootIdentifier::ThickFp => {
                        match role {
                            EntityRouteRole::StaticThinFpTyDecl { .. } => self.write("fn("),
                            _ => self.write("ThickFp<fn("),
                        }
                        for i in 0..(entity_route.spatial_arguments.len() - 1) {
                            if i > 0 {
                                self.write(", ")
                            }
                            let argument_ty = entity_route.spatial_arguments[i].take_entity_route();
                            if !self.db.is_copyable(argument_ty).unwrap() {
                                self.write("&")
                            }
                            self.gen_entity_route(argument_ty, EntityRouteRole::StaticDecl)
                        }
                        if entity_route.ident().as_str() == "downmost" {
                            p!(self.db.needs_eval_context(entity_route));
                            todo!()
                        }
                        match role {
                            EntityRouteRole::StaticThinFpTyDecl {
                                needs_eval_context: true,
                            } => {
                                if entity_route.spatial_arguments.len() > 1 {
                                    self.write(", ")
                                }
                                self.write("&dyn __EvalContext<'static>")
                            }
                            _ => (),
                        }
                        self.write(")");
                        let output_ty = entity_route
                            .spatial_arguments
                            .last()
                            .unwrap()
                            .take_entity_route();
                        if output_ty != EntityRoutePtr::Root(RootIdentifier::Void) {
                            self.write("->");
                            self.gen_entity_route(output_ty, EntityRouteRole::StaticDecl)
                        }
                        match role {
                            EntityRouteRole::StaticThinFpTyDecl { .. } => self.write(""),
                            _ => self.write(">"),
                        }
                        return;
                    }
                    RootIdentifier::FnMut => todo!(),
                    RootIdentifier::Fn => todo!(),
                    RootIdentifier::FnOnce => todo!(),
                    RootIdentifier::Ref => {
                        match role {
                            EntityRouteRole::StaticCallRoute
                            | EntityRouteRole::ForAnyLifetimeOther => self.write("&"),
                            EntityRouteRole::Caller
                            | EntityRouteRole::Decl
                            | EntityRouteRole::Other => self.write("&'eval "),
                            EntityRouteRole::StaticDecl => self.write("&'static "),
                            EntityRouteRole::StaticThinFpTyDecl { .. } => self.write("&'static "),
                            EntityRouteRole::FpValue => todo!(),
                        }
                        self.gen_entity_route(
                            entity_route.entity_route_argument(0),
                            role.argument_role(),
                        );
                        return;
                    }
                    _ => self.result += &ident,
                },
                EntityRouteVariant::Package { .. } => self.write("crate"),
                EntityRouteVariant::Child { parent, ident } => {
                    self.gen_entity_route(parent, role.parent_role());
                    self.write("::");
                    self.write(&ident);
                    // ad hoc
                    if ident.as_str() == "pop_with_largest_opt_f32" {
                        let elem_ty = parent.entity_route_argument(0);
                        if self.db.is_copyable(elem_ty).unwrap() {
                            self.write("_copyable")
                        } else {
                            self.write("_borrow")
                        }
                    }
                }
                EntityRouteVariant::TargetInputValue => self.write("__input"),
                EntityRouteVariant::Any { .. } => {
                    p!(entity_route);
                    todo!()
                }
                EntityRouteVariant::ThisType { .. } => todo!(),
                EntityRouteVariant::TypeAsTraitMember { .. } => todo!(),
                EntityRouteVariant::TargetOutputType => todo!(),
            }
        }
        let needs_eval_ref = match role {
            EntityRouteRole::Decl
            | EntityRouteRole::StaticThinFpTyDecl { .. }
            | EntityRouteRole::StaticDecl => {
                self.db.entity_route_variant_contains_eval_ref(entity_route)
            }
            _ => false,
        };
        if needs_eval_ref || entity_route.spatial_arguments.len() > 0 {
            match role {
                EntityRouteRole::Caller | EntityRouteRole::StaticCallRoute => self.write("::"),
                _ => (),
            }
            self.write("<");
            if needs_eval_ref {
                match role {
                    EntityRouteRole::Decl => self.write("'eval"),
                    EntityRouteRole::StaticThinFpTyDecl { .. } | EntityRouteRole::StaticDecl => {
                        self.write("'static")
                    }
                    _ => panic!(),
                }
            }
            for i in 0..entity_route.spatial_arguments.len() {
                if i > 0 || needs_eval_ref {
                    self.write(", ")
                }
                match entity_route.spatial_arguments[i] {
                    SpatialArgument::Const(_) => todo!(),
                    SpatialArgument::EntityRoute(entity_route) => {
                        self.gen_entity_route(entity_route, role.argument_role())
                    }
                }
            }
            self.write(">");
        }
    }
}
