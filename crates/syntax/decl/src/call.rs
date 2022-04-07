use entity_route::StaticEntityDecl;
use print_utils::msg_once;
use vm::InputContract;

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RoutineDecl {
    pub inputs: Vec<InputDecl>,
    pub output: EntityRoutePtr,
}

impl From<&RoutineDefnHead> for RoutineDecl {
    fn from(head: &RoutineDefnHead) -> Self {
        RoutineDecl {
            inputs: head
                .input_placeholders
                .iter()
                .map(|input_placeholder| InputDecl {
                    contract: input_placeholder.contract,
                    ty: input_placeholder.ranged_ty.route,
                })
                .collect(),
            output: head.output.route,
        }
    }
}

impl From<&MembRoutineDefnHead> for MembCallDecl {
    fn from(head: &MembRoutineDefnHead) -> Self {
        Self {
            this_contract: head.this_contract,
            inputs: head
                .input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.into())
                .collect(),
            output: head.output.route,
            generic_placeholders: head.generics.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InputDecl {
    pub contract: InputContract,
    pub ty: EntityRoutePtr,
}

impl InputDecl {
    pub fn from_static(db: &dyn DeclQueryGroup, input: &StaticInputDecl) -> Self {
        Self {
            ty: todo!(),
            contract: input.contract,
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        Self {
            ty: instantiator.instantiate_scope(self.ty).as_scope(),
            contract: self.contract,
        }
    }
}

impl Into<InputDecl> for &InputPlaceholder {
    fn into(self) -> InputDecl {
        InputDecl {
            contract: self.contract,
            ty: self.ranged_ty.route,
        }
    }
}

impl RoutineDecl {
    fn new_vec(ty: EntityRoutePtr) -> Self {
        msg_once!("new vec compiled");
        Self {
            inputs: Vec::new(),
            output: ty,
        }
    }
}

pub(crate) fn call_decl(
    db: &dyn DeclQueryGroup,
    scope: EntityRoutePtr,
) -> InferResultArc<RoutineDecl> {
    let source = db.entity_source(scope)?;
    return match source {
        EntitySource::Builtin(data) => Ok(Arc::new(match data.decl {
            StaticEntityDecl::Func(ref signature) => func_call_decl_from_raw(db, signature),
            StaticEntityDecl::TyTemplate => RoutineDecl::new_vec(scope),
            _ => panic!(),
        })),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.kind {
                AstKind::RoutineDefnHead(ref head) => Ok(Arc::new(head.into())),
                // type constructor
                AstKind::TypeDefnHead {
                    ref kind,
                    generic_placeholders: ref generics,
                    ..
                } => match kind {
                    RawTyKind::Enum => todo!(),
                    RawTyKind::Struct => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::MembVarDefn(ref memb_var_defn) => inputs.push(InputDecl {
                                    contract: memb_var_defn.contract.constructor_input(),
                                    ty: memb_var_defn.ty,
                                }),
                                _ => (),
                            }
                        }
                        msg_once!("struct type call compiled");
                        Ok(Arc::new(RoutineDecl {
                            inputs,
                            output: scope,
                        }))
                    }
                    RawTyKind::Record => {
                        let mut inputs = vec![];
                        for subitem in item.children.unwrap() {
                            let subast = subitem.value.as_ref()?;
                            match subast.kind {
                                AstKind::MembVarDefn(ref memb_var_defn) => inputs.push(InputDecl {
                                    contract: memb_var_defn.contract.constructor_input(),
                                    ty: memb_var_defn.ty,
                                }),
                                _ => (),
                            }
                        }
                        Ok(Arc::new(RoutineDecl {
                            inputs,
                            output: scope,
                        }))
                    }
                    RawTyKind::Primitive => todo!(),
                    RawTyKind::Vec => todo!(),
                    RawTyKind::Array => todo!(),
                    RawTyKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        EntitySource::Module { file: file_id } => todo!(),
        EntitySource::Contextual { .. } => todo!(),
    };

    fn func_call_decl_from_raw(
        this: &dyn DeclQueryGroup,
        signature: &StaticFuncDecl,
    ) -> RoutineDecl {
        let inputs = signature
            .inputs
            .iter()
            .map(|sig| {
                Ok(InputDecl {
                    ty: this.parse_ty(sig.ty)?,
                    contract: sig.contract,
                })
            })
            .collect::<AstResult<Vec<InputDecl>>>()
            .unwrap();
        let output = this.parse_ty(signature.output).unwrap();
        RoutineDecl { inputs, output }
    }
}