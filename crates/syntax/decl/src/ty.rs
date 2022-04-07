mod enum_ty;
mod impl_instantiate;
mod record;
mod vec;

use print_utils::msg_once;
pub use vec::*;

use crate::*;
use ast::AstIter;
use entity_route::*;
use enum_ty::*;
use record::*;
use syntax_types::EnumVariantKind;
use vec_map::VecDict;
use vm::{MembAccessContract, TySignature};
use word::{IdentMap, WordAllocator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TyDecl {
    pub generic_placeholders: IdentMap<GenericPlaceholder>,
    pub traits: Vec<EntityRoutePtr>,
    pub members: IdentMap<MembDecl>,
    pub kind: TyDeclKind,
}

impl TyDecl {
    fn new(
        db: &dyn DeclQueryGroup,
        generic_placeholders: IdentMap<GenericPlaceholder>,
        traits: Vec<EntityRoutePtr>,
        kind: TyDeclKind,
    ) -> Self {
        msg_once!("members from traits");
        let mut members = IdentMap::default();
        for trait_route in &traits {
            let trait_decl = db.trait_decl(*trait_route).unwrap();
            members.extends(&trait_decl.members)
        }
        match kind {
            TyDeclKind::Struct {
                ref memb_vars,
                ref memb_routines,
            } => {
                for (memb_ident, memb_access_decl) in memb_vars.iter() {
                    members.insert_new(
                        *memb_ident,
                        MembDecl {
                            variant: MembDeclVariant::Var(memb_access_decl.clone()),
                        },
                    )
                }
                for (memb_ident, memb_call_decl) in memb_routines.iter() {
                    members.insert_new(
                        *memb_ident,
                        MembDecl {
                            variant: MembDeclVariant::Routine(memb_call_decl.clone()),
                        },
                    )
                }
            }
            TyDeclKind::Enum { ref variants } => todo!(),
            TyDeclKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
            TyDeclKind::Vec { element_ty } => todo!(),
        };
        TyDecl {
            generic_placeholders,
            traits,
            members,
            kind,
        }
    }

    pub fn memb_idx(&self, memb_ident: CustomIdentifier) -> usize {
        match self.kind {
            TyDeclKind::Struct {
                ref memb_vars,
                ref memb_routines,
            } => memb_vars.position(memb_ident).unwrap(),
            TyDeclKind::Enum { ref variants } => todo!(),
            TyDeclKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
            TyDeclKind::Vec { element_ty } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TyDeclKind {
    Struct {
        memb_vars: Arc<IdentMap<MembAccessDecl>>,
        memb_routines: IdentMap<MembCallDecl>,
    },
    Enum {
        variants: IdentMap<EnumVariantDecl>,
    },
    Record {
        memb_vars: IdentMap<MembAccessDecl>,
        memb_features: IdentMap<EntityRoutePtr>,
    },
    Vec {
        element_ty: EntityRoutePtr,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MembAccessKind {
    StructMembVar,
    StructMembFeature,
    RecordMemb,
}

impl TyDecl {
    // fn vec(word_allocator: &WordAllocator, element_ty: ScopePtr) -> Self {
    //     let mut members = IdentMap::default();
    //     members.insert_new(
    //         word_allocator.alloc_from_ref("push").custom().unwrap(),
    //         MembSignature {
    //             kind: MembSignatureKind::Routine,
    //         },
    //     );
    //     Self {
    //         members,
    //         kind: TyDeclKind::Vec { element_ty },
    //     }
    // }

    pub fn memb_access_ty_result(&self, ident: CustomIdentifier) -> InferResult<EntityRoutePtr> {
        match self.kind {
            TyDeclKind::Struct { ref memb_vars, .. } => ok_or!(
                memb_vars.get(ident),
                format!("no such member variable {}", ident.0)
            )
            .map(|signature| signature.ty),
            TyDeclKind::Enum { ref variants } => todo!(),
            TyDeclKind::Record {
                ref memb_vars,
                ref memb_features,
            } => {
                if let Some(memb_var) = memb_vars.get(ident) {
                    Ok(memb_var.ty)
                } else if let Some(memb_feature) = memb_features.get(ident) {
                    Ok(*memb_feature)
                } else {
                    todo!()
                }
            }
            TyDeclKind::Vec { element_ty } => todo!(),
        }
    }

    pub fn memb_access_decl(&self, ident: CustomIdentifier) -> MembAccessDecl {
        match self.kind {
            TyDeclKind::Struct { ref memb_vars, .. } => *memb_vars.get(ident).unwrap(),
            TyDeclKind::Enum { ref variants } => todo!(),
            TyDeclKind::Record {
                ref memb_vars,
                ref memb_features,
            } => {
                if let Some(memb_var) = memb_vars.get(ident) {
                    *memb_var
                } else if let Some(memb_feature) = memb_features.get(ident) {
                    MembAccessDecl {
                        contract: MembAccessContract::LazyOwn,
                        ty: *memb_feature,
                    }
                } else {
                    todo!()
                }
            }
            TyDeclKind::Vec { element_ty } => todo!(),
        }
    }

    pub fn memb_access_kind(&self, memb_ident: CustomIdentifier) -> MembAccessKind {
        match self.kind {
            TyDeclKind::Struct {
                ref memb_vars,
                ref memb_routines,
            } => {
                if memb_vars.get(memb_ident).is_some() {
                    MembAccessKind::StructMembVar
                } else {
                    panic!("todo: memb feature of struct")
                }
            }
            TyDeclKind::Enum { ref variants } => todo!(),
            TyDeclKind::Record {
                ref memb_vars,
                ref memb_features,
            } => {
                if memb_vars.get(memb_ident).is_some() {
                    MembAccessKind::RecordMemb
                } else if memb_features.get(memb_ident).is_some() {
                    MembAccessKind::RecordMemb
                } else {
                    todo!()
                }
            }
            TyDeclKind::Vec { element_ty } => todo!(),
        }
    }

    pub fn signature(&self) -> TySignature {
        match self.kind {
            TyDeclKind::Struct { ref memb_vars, .. } => {
                let mut vm_memb_vars = IdentMap::<MembAccessContract>::default();
                memb_vars.iter().for_each(|(ident, memb_var_sig)| {
                    vm_memb_vars.insert_new(*ident, memb_var_sig.contract)
                });
                TySignature::Struct {
                    memb_vars: vm_memb_vars,
                }
            }
            TyDeclKind::Enum { ref variants } => todo!(),
            TyDeclKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
            TyDeclKind::Vec { element_ty } => TySignature::Vec,
        }
    }

    pub fn memb_call_decl(&self, ident: CustomIdentifier) -> InferResult<&MembCallDecl> {
        match self.members.get(ident) {
            Some(memb_decl) => match memb_decl.variant {
                MembDeclVariant::Var(_) => todo!(),
                MembDeclVariant::Routine(ref signature) => Ok(signature),
            },
            None => err!(format!("no member named {}", &ident)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDecl {
    Constant,
}

pub(crate) fn ty_decl(db: &dyn DeclQueryGroup, scope: EntityRoutePtr) -> InferResultArc<TyDecl> {
    let source = db.entity_source(scope)?;
    match source {
        EntitySource::Builtin(data) => Ok(Arc::new(match data.decl {
            StaticEntityDecl::Func(_) => todo!(),
            StaticEntityDecl::Module => todo!(),
            StaticEntityDecl::Ty { .. } => todo!(),
            StaticEntityDecl::TyTemplate => {
                let vec_decl_template = db.vec_decl();
                vec_decl_template.instantiate(db, &scope.generics)
            }
            StaticEntityDecl::Trait { .. } => todo!(),
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
                AstKind::TypeDefnHead {
                    kind,
                    ref generic_placeholders,
                    ..
                } => match kind {
                    RawTyKind::Enum => enum_decl(
                        generic_placeholders.clone(),
                        derived_not_none!(item.children)?,
                    ),
                    RawTyKind::Struct => {
                        struct_decl(db, generic_placeholders.clone(), item.children.unwrap())
                    }
                    RawTyKind::Record => {
                        record_decl(generic_placeholders.clone(), item.children.unwrap())
                    }
                    RawTyKind::Primitive => todo!(),
                    RawTyKind::Vec => todo!(),
                    RawTyKind::Array => todo!(),
                    RawTyKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        EntitySource::Module { file } => todo!(),
        EntitySource::Contextual { .. } => todo!(),
    }
}

pub(crate) fn struct_decl(
    db: &dyn DeclQueryGroup,
    generic_placeholders: IdentMap<GenericPlaceholder>,
    children: AstIter,
) -> InferResultArc<TyDecl> {
    let mut memb_vars = VecDict::default();
    let mut memb_routines = VecDict::default();
    let mut traits = vec![db.entity_route_menu().clone_trait];

    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVarDefn(ref memb_var_defn) => memb_vars.insert_new(
                memb_var_defn.ident,
                MembAccessDecl {
                    contract: memb_var_defn.contract,
                    ty: memb_var_defn.ty,
                },
            ),
            AstKind::MembRoutineDefnHead(ref memb_var_defn) => {
                memb_routines.insert_new(memb_var_defn.ident, memb_var_defn.into())
            }
            _ => panic!(),
        }
    }

    Ok(Arc::new(TyDecl::new(
        db,
        generic_placeholders,
        traits,
        TyDeclKind::Struct {
            memb_vars: Arc::new(memb_vars),
            memb_routines,
        },
    )))
}