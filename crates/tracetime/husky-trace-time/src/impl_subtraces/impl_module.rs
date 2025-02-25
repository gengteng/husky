use crate::*;
use husky_entity_kind::EntityKind;
use husky_entity_route::EntityRoutePtr;

impl HuskyTracetime {
    pub(super) fn module_subtraces(
        &mut self,
        trace: &Trace,
        module: EntityRoutePtr,
    ) -> Vec<TraceId> {
        let mut subtrace_ids = vec![];
        let module_file = self.comptime().module_file(module).unwrap();
        for (subentity_kind, subentity_route) in
            self.comptime().subentity_kinded_routes(module).iter()
        {
            match subentity_kind {
                EntityKind::Module => {
                    if self.comptime().module_contains_features(*subentity_route) {
                        subtrace_ids.push(self.new_trace(
                            Some(trace.id()),
                            0,
                            TraceVariant::Module {
                                route: *subentity_route,
                                file: module_file,
                                range: Default::default(),
                            },
                        ))
                    }
                }
                EntityKind::Feature => {
                    let repr = self.runtime().entity_feature_repr(*subentity_route);
                    subtrace_ids.push(self.new_trace(
                        Some(trace.id()),
                        0,
                        TraceVariant::EntityFeature {
                            route: *subentity_route,
                            repr,
                        },
                    ))
                }
                _ => (),
            }
        }
        subtrace_ids
    }
}
