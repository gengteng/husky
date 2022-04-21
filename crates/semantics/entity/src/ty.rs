mod member;

use atom::{symbol::SymbolContextKind, SymbolContext};
use infer_decl::MemberIdx;
pub use member::*;
use print_utils::msg_once;
use static_defn::StaticTypeDefn;

use std::{iter::Peekable, sync::Arc};

use super::*;
use ast::*;
use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr};
use file::FilePtr;
use infer_total::InferQueryGroup;
use semantics_eager::{FuncStmt, ProcStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use vec_dict::VecDict;
use word::{CustomIdentifier, IdentDict};

impl EntityDefnVariant {
    pub(crate) fn ty_from_ast(
        db: &dyn InferQueryGroup,
        entity_route: EntityRoutePtr,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        let (ident, kind, generic_placeholders) = match head.kind {
            AstKind::TypeDefnHead {
                ident,
                kind,
                ref generic_placeholders,
            } => (ident, kind, generic_placeholders.clone()),
            _ => panic!(),
        };
        let mut children = children.peekable();
        let mut type_members = IdentDict::default();
        let mut trait_impls = Vec::new();
        msg_once!("todo");

        let variants = match kind {
            TyKind::Enum => Self::collect_variants(db, file, entity_route, &mut children)?,
            _ => Default::default(),
        };
        Self::collect_fields(
            db,
            arena,
            file,
            &mut children,
            &mut type_members,
            entity_route,
        )?;
        Self::collect_other_members(db, arena, file, entity_route, children, &mut type_members)?;
        Ok(EntityDefnVariant::new_ty(
            type_members,
            variants,
            kind,
            trait_impls,
        ))
    }

    fn new_ty(
        type_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TyKind,
        trait_impls: Vec<Arc<TraitImplDefn>>,
    ) -> Self {
        let members = collect_all_members(&type_members, &trait_impls);
        EntityDefnVariant::Type {
            type_members,
            variants,
            kind,
            trait_impls,
            members,
        }
    }

    pub(crate) fn ty_from_static(
        db: &dyn EntityDefnQueryGroup,
        ty: EntityRoutePtr,
        type_defn: &StaticTypeDefn,
    ) -> Self {
        let type_members = type_defn.type_members.map(|_| todo!());
        let variants = type_defn.variants.map(|_| todo!());
        let kind = type_defn.kind;
        let symbol_context = SymbolContext {
            opt_package_main: None,
            db: db.upcast(),
            opt_this_ty: Some(ty),
            symbols: &[],
            kind: SymbolContextKind::Normal,
        };
        let trait_impls = type_defn
            .trait_impls
            .map(|trait_impl| TraitImplDefn::from_static(&symbol_context, trait_impl));
        Self::new_ty(type_members, variants, kind, trait_impls)
    }

    fn collect_variants(
        db: &dyn InferQueryGroup,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
    ) -> SemanticResult<IdentDict<Arc<EntityDefn>>> {
        let mut variants = VecDict::default();
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref()?;
            match ast.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: raw_variant_kind,
                } => {
                    variants.insert_new(EntityDefn::new(
                        ident.into(),
                        EntityDefnVariant::EnumVariant {
                            ident,
                            variant: match raw_variant_kind {
                                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
                            },
                        },
                        db.intern_entity_route(EntityRoute {
                            kind: EntityRouteKind::Child {
                                parent: ty_route,
                                ident,
                            },
                            generic_arguments: vec![],
                        }),
                        file,
                        ast.range,
                    ));
                    children.next();
                }
                _ => break,
            }
        }
        Ok(variants)
    }

    fn record_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        todo!()
        // let mut fields = VecDict::default();
        // for subitem in children {
        //     match subitem.value.as_ref()?.kind {
        //         AstKind::Use { ident, scope } => (),
        //         AstKind::RoutineDefnHead(_) => todo!(),
        //         AstKind::FieldDefn(ref field_var_defn) => fields.insert_new(field_var_defn.clone()),
        //         AstKind::MembFeatureDefnHead { ident, ty } => {
        //             let stmts = semantics_lazy::parse_lazy_stmts(
        //                 &[],
        //                 db,
        //                 arena,
        //                 subitem.children.unwrap(),
        //                 file,
        //             )?;
        //             fields.insert_new(FieldDefn {
        //                 ident,
        //                 output_ty: ty,
        //                 stmts,
        //             });
        //         }
        //         _ => panic!(),
        //     }
        // }
        // Ok(TyKind::Record { fields })
    }

    pub fn method(&self, member_idx: usize) -> &Arc<EntityDefn> {
        todo!()
        // match self.members[member_idx] {
        //     MemberDefn::TypeField(_) => todo!(),
        //     MemberDefn::TypeMethod(_) => todo!(),
        // }
    }
}

impl EntityDefn {
    pub fn method(&self, member_idx: MemberIdx) -> &Arc<EntityDefn> {
        match self.variant {
            EntityDefnVariant::Type { ref members, .. } => &members[member_idx.0 as usize],
            EntityDefnVariant::EnumVariant { ident, ref variant } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MethodKind {
    Func { stmts: Arc<Vec<Arc<FuncStmt>>> },
    Proc { stmts: Arc<Vec<Arc<ProcStmt>>> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDefnVariant {
    Constant,
}

impl EntityDefnVariant {
    pub fn enum_variant(
        db: &dyn EntityDefnQueryGroup,
        ident: CustomIdentifier,
        enum_variant_kind: EnumVariantKind,
        children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant {
            ident,
            variant: match enum_variant_kind {
                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
            },
        }
    }
}