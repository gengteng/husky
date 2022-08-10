use crate::*;
use husky_entity_route::RangedEntityRoute;
use husky_entity_semantics::{Visualizer, VisualizerVariant};
use husky_word::RootIdentifier;
use vm::__VMResult;

pub(crate) fn visual_feature_lazy_block(
    db: &dyn FeatureGenQueryGroup,
    this: FeatureRepr,
) -> __VMResult<Arc<FeatureLazyBlock>> {
    let visualizer: Arc<Visualizer> = db.comptime().visualizer(this.ty());
    Ok(FeatureLazyBlock::new(
        db,
        Some(this),
        match visualizer.variant {
            VisualizerVariant::Custom { ref stmts } => stmts,
            _ => panic!(),
        },
        &[],
        None,
        db.feature_interner(),
        RangedEntityRoute {
            route: RootIdentifier::VisualType.into(),
            range: Default::default(),
        },
    ))
}