use entity_kind::EntityKind;

use crate::*;

pub(crate) fn needs_eval_context(
    db: &dyn RustCodeGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> bool {
    let entity_link_dependees = db.entity_link_dependees(entity_route);
    for link_route in entity_link_dependees.iter() {
        let link_entity_kind = db.entity_kind(*link_route).unwrap();
        match link_entity_kind {
            EntityKind::Feature => return true,
            EntityKind::Main => panic!(),
            _ => (),
        }
    }
    false
}