mod field;
mod method;

pub use field::*;
use husky_eager_semantics::{parse_func_stmts, parse_proc_stmts};
use husky_word::Paradigm;
pub use method::*;

use super::*;

pub fn collect_all_members(
    type_members: &[Arc<EntityDefn>],
    trait_impls: &[Arc<TraitImplDefn>],
) -> Avec<EntityDefn> {
    let mut members = type_members.to_vec();
    for trait_impl in trait_impls {
        members.extend(
            trait_impl
                .member_impls
                .iter()
                .map(|member_impl| member_impl.clone()),
        );
    }
    Arc::new(members)
}

pub fn member_defn(db: &dyn EntityDefnQueryGroup, member_route: EntityRoutePtr) -> Arc<EntityDefn> {
    let ty = member_route.parent();
    let ty_defn = db.entity_defn(ty).unwrap();
    let member_idx = db.member_idx(member_route);
    match ty_defn.variant {
        EntityDefnVariant::Ty { ref members, .. } => members[member_idx.0 as usize].clone(),
        _ => panic!(),
    }
}

impl EntityDefnVariant {
    pub(crate) fn collect_other_ty_members(
        db: &dyn EntityDefnQueryGroup,
        arena: &RawExprArena,
        file: FilePtr,
        ty_route: EntityRoutePtr,
        children: &mut Peekable<AstIter>,
        members: &mut IdentDict<Arc<EntityDefn>>,
    ) -> SemanticResult<()> {
        while let Some(child) = children.peek() {
            let ast = child.value.as_ref().unwrap();
            let (ident, variant): (CustomIdentifier, _) = match ast.variant {
                AstVariant::TypeDefnHead { .. } => todo!(),
                AstVariant::MainDefnHead => todo!(),
                AstVariant::CallFormDefnHead {
                    opt_this_liason,
                    paradigm,
                    ident,
                    ref spatial_parameters,
                    ref parameters,
                    return_ty: output_ty,
                    ..
                } => match opt_this_liason {
                    Some(this_contract) => {
                        let method_source = match paradigm {
                            Paradigm::EagerProcedural => CallFormSource::Proc {
                                stmts: parse_proc_stmts(
                                    db.upcast(),
                                    arena,
                                    child.opt_children.clone().unwrap(),
                                    file,
                                )?,
                            },
                            Paradigm::EagerFunctional => CallFormSource::Func {
                                stmts: parse_func_stmts(
                                    db.upcast(),
                                    arena,
                                    child.opt_children.clone().unwrap(),
                                    file,
                                )?,
                            },
                            Paradigm::LazyFunctional => todo!(),
                        };
                        (
                            ident.ident,
                            EntityDefnVariant::Method {
                                parameters: parameters.clone(),
                                output_ty,
                                this_modifier: this_contract,
                                output_modifier: OutputModifier::Transfer,
                                spatial_parameters: spatial_parameters.clone(),
                                method_defn_kind: MethodDefnKind::TypeMethod { ty: ty_route },
                                opt_source: Some(method_source),
                            },
                        )
                    }
                    None => (
                        ident.ident,
                        EntityDefnVariant::function(
                            db,
                            ast,
                            child.opt_children.clone().unwrap(),
                            arena,
                            file,
                        )?,
                    ),
                },
                AstVariant::FeatureDefnHead { .. } => todo!(),
                AstVariant::Use { .. } => todo!(),
                AstVariant::FieldDefnHead { ranged_ident, .. } => (
                    ranged_ident.ident,
                    EntityDefnVariant::ty_field_from_ast(
                        db,
                        arena,
                        file,
                        ty_route,
                        ast,
                        child.opt_children.clone(),
                    )?,
                ),
                AstVariant::DatasetConfigDefnHead => todo!(),
                AstVariant::Stmt(_) => todo!(),
                AstVariant::EnumVariantDefnHead { .. } => todo!(),
                AstVariant::Submodule { .. } => todo!(),
                AstVariant::Visual => break,
            };
            children.next();
            members
                .insert_new(EntityDefn::new(
                    db,
                    ident.into(),
                    variant,
                    db.subroute(ty_route, ident, thin_vec![]),
                    file,
                    ast.range,
                ))
                .unwrap()
        }
        Ok(())
    }
}
