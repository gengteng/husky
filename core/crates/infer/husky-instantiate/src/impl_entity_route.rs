use crate::*;

impl Instantiable for EntityRoutePtr {
    type Target = SpatialArgument;

    fn instantiate(&self, ctx: &InstantiationContext) -> SpatialArgument {
        match ctx.db.entity_kind(*self).unwrap() {
            EntityKind::Module => SpatialArgument::EntityRoute(*self),
            EntityKind::EnumLiteral => todo!(),
            _ => {
                let (kind, mut generics) = match self.kind {
                    EntityRouteKind::Package { .. } => panic!(),
                    EntityRouteKind::Root { ident } => (self.kind, thin_vec![]),
                    EntityRouteKind::Child { parent, ident } => (
                        EntityRouteKind::Child {
                            parent: parent.instantiate(ctx).take_entity_route(),
                            ident,
                        },
                        thin_vec![],
                    ),
                    EntityRouteKind::Input { main } => todo!(),
                    EntityRouteKind::Generic { ident, .. } => {
                        if let Some(idx) = ctx.find_generic(ident) {
                            match ctx.spatial_arguments[idx] {
                                SpatialArgument::Const(value) => {
                                    should_eq!(self.spatial_arguments.len(), 0);
                                    return SpatialArgument::Const(value);
                                }
                                SpatialArgument::EntityRoute(scope) => {
                                    (scope.kind, scope.spatial_arguments.clone())
                                }
                            }
                        } else {
                            p!(ident, ctx.spatial_parameters);
                            todo!()
                        }
                    }
                    EntityRouteKind::ThisType => (EntityRouteKind::ThisType, thin_vec![]),
                    EntityRouteKind::TypeAsTraitMember {
                        ty: parent,
                        trai,
                        ident,
                    } => todo!(),
                };
                // convention: A<B,C> = A<B><C>
                generics.extend(self.spatial_arguments.instantiate(ctx));
                SpatialArgument::EntityRoute(ctx.db.intern_entity_route(EntityRoute {
                    kind,
                    temporal_arguments: thin_vec![],
                    spatial_arguments: generics,
                }))
            }
        }
    }
}