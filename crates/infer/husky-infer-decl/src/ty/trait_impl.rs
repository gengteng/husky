use crate::*;
use husky_atom::AtomContext;
use husky_entity_kind::{FieldKind, MemberKind};
use husky_implement::{Implementable, ImplementationContext};
use husky_print_utils::msg_once;
use map_collect::MapCollect;
use thin_vec::thin_vec;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitImplDecl {
    trai: EntityRoutePtr,
    this_ty: EntityRoutePtr,
    member_impls: Vec<TraitMemberImplDecl>,
}

impl TraitImplDecl {
    pub(crate) fn new(
        trai: EntityRoutePtr,
        this_ty: EntityRoutePtr,
        member_impls: Vec<TraitMemberImplDecl>,
    ) -> Self {
        assert!(!this_ty.is_self_ty_alias());
        Self {
            trai,
            this_ty,
            member_impls,
        }
    }

    pub(crate) fn from_static(
        db: &dyn DeclQueryGroup,
        static_trait_impl: &StaticTraitImplDefn,
        symbol_context: &mut dyn AtomContext,
    ) -> Arc<Self> {
        let trait_route = symbol_context
            .parse_entity_route(static_trait_impl.trai)
            .unwrap();
        let trait_decl = db.trait_decl(trait_route).unwrap();
        let member_impls = TraitMemberImplDecl::collect_from_static(
            db,
            symbol_context,
            &trait_decl,
            static_trait_impl.member_impls,
        );
        Arc::new(Self::new(
            trait_route,
            symbol_context.opt_this_ty().unwrap(),
            member_impls,
        ))
    }

    pub fn trai(&self) -> EntityRoutePtr {
        self.trai
    }

    pub fn this_ty(&self) -> EntityRoutePtr {
        self.this_ty
    }

    pub fn member_impls(&self) -> &[TraitMemberImplDecl] {
        &self.member_impls
    }

    pub(crate) fn instantiate(&self, ctx: &InstantiationContext) -> Arc<Self> {
        Arc::new(Self {
            trai: self.trai.instantiate(ctx).take_entity_route(),
            this_ty: self.this_ty.instantiate(ctx).take_entity_route(),
            member_impls: self.member_impls.map(|member| member.instantiate(ctx)),
        })
    }

    pub(crate) fn member(&self, ident: CustomIdentifier) -> Option<&TraitMemberImplDecl> {
        self.member_impls
            .iter()
            .find(|impl_decl| impl_decl.ident() == ident)
    }

    pub(crate) fn implicit_trait_impls(
        db: &dyn DeclQueryGroup,
        this_ty: EntityRoutePtr,
        ty_kind: TyKind,
        ty_members: &[TyMemberDecl],
        variants: &[EnumVariantDecl],
    ) -> InferResult<Vec<Arc<TraitImplDecl>>> {
        let mut trait_impl_decls = Vec::new();
        let entity_route_menu = db.entity_route_menu();
        let is_copyable = derive_is_copyable(db, ty_kind, ty_members, variants);
        if is_copyable {
            trait_impl_decls.push(Arc::new(TraitImplDecl {
                trai: entity_route_menu.copy_trait,
                this_ty,
                member_impls: Vec::new(),
            }))
        }
        if derive_is_clonable(db, is_copyable, this_ty, ty_kind, ty_members, variants)? {
            msg_once!("much to do here");
            let clone_trait = entity_route_menu.clone_trait;
            trait_impl_decls.push(Arc::new(TraitImplDecl {
                trai: clone_trait,
                this_ty,
                member_impls: vec![TraitMemberImplDecl::Method(Arc::new(CallFormDecl {
                    opt_route: Some(db.ty_as_trai_subroute(
                        this_ty,
                        clone_trait,
                        db.intern_word("clone").custom(),
                        thin_vec![],
                    )),
                    opt_this_liason: Some(ParameterModifier::None),
                    primary_parameters: Default::default(),
                    output: OutputDecl::new(db, OutputModifier::Transfer, this_ty)?,
                    spatial_parameters: Default::default(),
                    is_lazy: false,
                    variadic_parameters: VariadicParametersDecl::None,
                    keyword_parameters: Default::default(),
                }))],
            }))
        }
        msg_once!("handle other traits, PartialEq, Eq");
        Ok(trait_impl_decls)
    }
}

fn derive_is_copyable(
    db: &dyn DeclQueryGroup,
    ty_kind: TyKind,
    ty_members: &[TyMemberDecl],
    variants: &[EnumVariantDecl],
) -> bool {
    match ty_kind {
        TyKind::Enum => {
            for variant in variants {
                match variant.variant {
                    EnumVariantDeclVariant::Constant => (),
                }
            }
            true
        }
        TyKind::Record => false,
        TyKind::Struct => false,
        TyKind::Primitive => true,
        TyKind::Vec => false,
        TyKind::Array => false,
        TyKind::Slice => true,
        TyKind::CyclicSlice => false,
        TyKind::Tuple => todo!(),
        TyKind::Mor => todo!(),
        TyKind::ThickFp => true,
        TyKind::AssociatedAny => todo!(),
        TyKind::TargetOutputAny => todo!(),
        TyKind::ThisAny => todo!(),
        TyKind::SpatialPlaceholderAny => todo!(),
        TyKind::BoxAny => todo!(),
        TyKind::HigherKind => true,
        TyKind::Ref => todo!(),
        TyKind::Option => todo!(),
    }
}

fn derive_is_clonable(
    db: &dyn DeclQueryGroup,
    is_copyable: bool,
    this_ty: EntityRoutePtr,
    ty_kind: TyKind,
    ty_members: &[TyMemberDecl],
    variants: &[EnumVariantDecl],
) -> InferResult<bool> {
    // in husky, if a type is copyable, it's not clonable
    if is_copyable {
        return Ok(false);
    }
    Ok(match ty_kind {
        TyKind::Enum => {
            for variant in variants {
                match variant.variant {
                    EnumVariantDeclVariant::Constant => (),
                }
            }
            true
        }
        TyKind::Record => false,
        TyKind::Struct => {
            for ty_member in ty_members {
                match ty_member {
                    TyMemberDecl::Field(field) => {
                        if field.field_kind == FieldKind::StructRegular {
                            if !db.is_copyable(field.ty)? && !db.is_clonable(field.ty)? {
                                return Ok(false);
                            }
                        }
                    }
                    TyMemberDecl::Method(_) | TyMemberDecl::Call(_) => (),
                }
            }
            true
        }
        TyKind::Primitive => todo!(),
        TyKind::Vec => {
            msg_once!("Vec<E>, E should be copyable or clonable");
            true
        }
        TyKind::Array => true,
        TyKind::Slice => todo!(),
        TyKind::CyclicSlice => true,
        TyKind::Tuple => todo!(),
        TyKind::Mor => todo!(),
        TyKind::ThickFp => todo!(),
        TyKind::AssociatedAny => todo!(),
        TyKind::TargetOutputAny => todo!(),
        TyKind::ThisAny => todo!(),
        TyKind::SpatialPlaceholderAny => todo!(),
        TyKind::BoxAny => todo!(),
        TyKind::HigherKind => todo!(),
        TyKind::Ref => todo!(),
        TyKind::Option => todo!(),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberImplDecl {
    Method(Arc<CallFormDecl>),
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
            TraitMemberImplDecl::Method(call_form_decl) => call_form_decl.ident(),
            TraitMemberImplDecl::AssociatedType { ident, .. } => *ident,
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }

    pub fn kind(&self) -> MemberKind {
        match self {
            TraitMemberImplDecl::Method(call_form_decl) => MemberKind::Method {
                is_lazy: call_form_decl.is_lazy,
            },
            TraitMemberImplDecl::AssociatedType { .. } => MemberKind::TraitAssociatedType,
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => MemberKind::TraitAssociatedConstSize,
        }
    }

    pub fn generic_argument(&self) -> SpatialArgument {
        match self {
            TraitMemberImplDecl::Method(_) => todo!(),
            TraitMemberImplDecl::AssociatedType { ident, ty } => SpatialArgument::EntityRoute(*ty),
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }

    pub fn collect_from_static(
        db: &dyn DeclQueryGroup,
        symbol_context: &mut dyn AtomContext,
        trait_decl: &TraitDecl,
        static_member_impls: &[EntityStaticDefn],
    ) -> Vec<Self> {
        let member_symbol_impls: Vec<_> = static_member_impls
            .iter()
            .filter_map(|static_member_impl| match static_member_impl.variant {
                EntityStaticDefnVariant::Method { .. } => None,
                EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
                EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty } => Some((
                    db.intern_word(static_member_impl.name).custom(),
                    SpatialArgument::EntityRoute(symbol_context.parse_entity_route(ty).unwrap()),
                )),
                EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
                _ => panic!(),
            })
            .collect();
        let this_ty = symbol_context.opt_this_ty().unwrap();
        let implementor = ImplementationContext::new(db.upcast(), this_ty, &member_symbol_impls);

        trait_decl
            .members
            .map(|trait_member_decl| trait_member_decl.implement(&implementor))
    }

    pub fn instantiate(&self, ctx: &InstantiationContext) -> Self {
        match self {
            TraitMemberImplDecl::Method(call_form_decl) => {
                TraitMemberImplDecl::Method(call_form_decl.instantiate(ctx))
            }
            TraitMemberImplDecl::AssociatedType { ident, ty } => {
                TraitMemberImplDecl::AssociatedType {
                    ident: *ident,
                    ty: ty.instantiate(ctx).take_entity_route(),
                }
            }
            TraitMemberImplDecl::Call {} => todo!(),
            TraitMemberImplDecl::AssociatedConstSize {} => todo!(),
        }
    }
}
