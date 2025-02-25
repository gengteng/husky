#[cfg(test)]
use crate::*;
#[cfg(test)]
use husky_entity_route::SpatialArgument;
#[cfg(test)]
use husky_word::RootIdentifier;
#[cfg(test)]
use thin_vec::thin_vec;

#[test]
fn test_vec_ty_decl() {
    let db = HuskyComptime::new_default(__resolve_root_defn);
    let _vec_i32_route = db.route_call(
        EntityRoutePtr::Root(RootIdentifier::Vec),
        thin_vec![SpatialArgument::EntityRoute(db.entity_route_menu().i32_ty)],
    );
    let _vec_ty_decl = db.ty_decl(db.entity_route_menu().vec_ty).unwrap();
}
