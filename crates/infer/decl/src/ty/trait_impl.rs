use atom::{
    symbol::{Symbol, SymbolContextKind},
    SymbolContext,
};
use entity_kind::MemberKind;
use implement::Implementor;
use map_collect::MapCollect;
use print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDecl {
    pub trait_route: EntityRoutePtr,
    pub this_ty: EntityRoutePtr,
    pub member_impls: Vec<TraitMemberImplDecl>,
}

impl TraitImplDecl {
    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        static_trait_impl: &StaticTraitImplDefn,
        symbol_context: &SymbolContext,
    ) -> Arc<Self> {
        let trait_route = symbol_context
            .entity_route_from_str(static_trait_impl.route)
            .unwrap();
        let trait_decl = db.trait_decl(trait_route).unwrap();
        let member_impls = TraitMemberImplDecl::collect_from_static(
            db,
            symbol_context,
            &trait_decl,
            static_trait_impl.member_impls,
        );
        Arc::new(Self {
            trait_route,
            this_ty: symbol_context.opt_this_ty.unwrap(),
            member_impls,
        })
        // Self::from_trait_decl(db, trait_decl, symbol_context.opt_this_ty.unwrap())
    }

    // pub(crate) fn from_trait_decl(
    //     db: &dyn DeclQueryGroup,
    //     trait_decl: Arc<TraitDecl>,
    //     this_ty: EntityRoutePtr,
    // ) -> Arc<Self> {
    //     let memb_impls = trait_decl
    //         .members
    //         .map(|member| member.implement(db, this_ty));
    //     Arc::new(Self {
    //         trait_decl,
    //         this_ty,
    //         members: memb_impls,
    //     })
    // }

    pub(crate) fn clone_trait_impl(db: &dyn DeclQueryGroup, this_ty: EntityRoutePtr) -> Arc<Self> {
        todo!()
        // Self::from_trait_decl(db, db.trait_decl_menu().clone_trait.clone(), this_ty)
    }

    pub(crate) fn instantiate(&self, instantiator: &Instantiator) -> Arc<Self> {
        Arc::new(Self {
            trait_route: instantiator
                .instantiate_entity_route(self.trait_route)
                .as_scope(),
            this_ty: instantiator
                .instantiate_entity_route(self.this_ty)
                .as_scope(),
            member_impls: self
                .member_impls
                .map(|member| member.instantiate(instantiator)),
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberImplDecl {
    Method(Arc<MethodDecl>),
    AssociatedType {
        ident: CustomIdentifier,
        ty: EntityRoutePtr,
    },
    Call {},
    AssociatedConstSize {},
}

impl TraitMemberImplDecl {
    pub fn ident(&self) -> CustomIdentifier {
        match self {
            TraitMemberImplDecl::Method(method_decl) => method_decl.ident,
            TraitMemberImplDecl::AssociatedType { ident, .. } => *ident,
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }

    pub fn kind(&self) -> MemberKind {
        match self {
            TraitMemberImplDecl::Method(_) => MemberKind::Method,
            TraitMemberImplDecl::AssociatedType { .. } => MemberKind::AssociatedType,
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => MemberKind::AssociatedConstSize,
        }
    }

    pub fn generic_argument(&self) -> GenericArgument {
        match self {
            TraitMemberImplDecl::Method(_) => todo!(),
            TraitMemberImplDecl::AssociatedType { ident, ty } => GenericArgument::EntityRoute(*ty),
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }

    pub fn collect_from_static(
        db: &dyn DeclQueryGroup,
        symbol_context: &SymbolContext,
        trait_decl: &TraitDecl,
        static_member_impls: &[StaticTraitMemberImplDefn],
    ) -> Vec<Self> {
        let member_impl_context: Vec<_> = static_member_impls
            .iter()
            .filter_map(|static_member_impl| match static_member_impl.variant {
                StaticTraitMemberImplDefnVariant::Type { route } => Some((
                    db.intern_word(static_member_impl.name).custom(),
                    GenericArgument::EntityRoute(
                        symbol_context.entity_route_from_str(route).unwrap(),
                    ),
                )),
                StaticTraitMemberImplDefnVariant::Method { .. } => None,
            })
            .collect();
        // let member_impl_context: Vec<_> =
        //     member_impls.map(|member_impl| (member_impl.ident(), member_impl.generic_argument()));
        let this_ty = symbol_context.opt_this_ty.unwrap();
        let implementor = Implementor::new(db.upcast(), this_ty, &member_impl_context);

        trait_decl
            .members
            .map(|trait_member_decl| trait_member_decl.implement(db, &implementor))
        //     match trait_member_decl {
        //     TraitMemberDecl::Method(method_decl) => {
        //         TraitMemberImplDecl::Method(method_decl.implement(&implementor))
        //     }
        //     TraitMemberDecl::Type { ident, traits } => {
        //         TraitMemberImplDecl::AssociatedType { ident: (), ty: () }
        //     }
        //     TraitMemberDecl::ConstSize(_) => todo!(),
        //     TraitMemberDecl::Call {} => todo!(),
        // }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TraitMemberImplDecl::Method(method_decl) => {
                TraitMemberImplDecl::Method(method_decl.instantiate(instantiator))
            }
            TraitMemberImplDecl::AssociatedType { ident, ty } => {
                TraitMemberImplDecl::AssociatedType {
                    ident: *ident,
                    ty: instantiator.instantiate_entity_route(*ty).as_scope(),
                }
            }
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }
}