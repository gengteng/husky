use entity_route::SpatialArgument;
use word::RootIdentifier;

use crate::*;

#[test]
fn test_vec_ty_decl() {
    let db = HuskyCompileTime::default();
    let _vec_i32_route = db.make_route(
        EntityRoutePtr::Root(RootIdentifier::Vec),
        vec![SpatialArgument::EntityRoute(db.entity_route_menu().i32_ty)],
    );
    let _vec_ty_decl = db.ty_decl(db.entity_route_menu().vec_ty).unwrap();
}