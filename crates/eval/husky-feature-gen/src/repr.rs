use husky_entity_semantics::DefinitionRepr;
use husky_file::FilePtr;
use husky_instruction_gen::new_func_instruction_sheet;
use husky_linkage_table::ResolveLinkage;
use vm::{__LinkageFp, __Register, __VMResult};

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FeatureRepr {
    Value {
        value: __Register<'static>,
        ty: EntityRoutePtr,
        file: FilePtr,
        range: TextRange,
        feature: FeaturePtr,
    },
    Expr(Arc<FeatureExpr>),
    LazyBlock(Arc<FeatureLazyBlock>),
    FuncBlock(Arc<FeatureFuncBlock>),
    ProcBlock(Arc<FeatureProcBlock>),
    TargetInput {
        ty: EntityRoutePtr,
        feature: FeaturePtr,
    },
}

impl FeatureRepr {
    pub fn is_lazy(&self) -> bool {
        match self {
            FeatureRepr::Expr(_) => true,
            FeatureRepr::LazyBlock(_) => true,
            FeatureRepr::Value { .. } => false,
            FeatureRepr::FuncBlock(_) => false,
            FeatureRepr::ProcBlock(_) => false,
            FeatureRepr::TargetInput { .. } => false,
        }
    }

    pub fn ty(&self) -> EntityRoutePtr {
        match self {
            FeatureRepr::Value { ty, .. } => *ty,
            FeatureRepr::Expr(expr) => expr.expr.ty(),
            FeatureRepr::LazyBlock(block) => block.ty.route,
            FeatureRepr::FuncBlock(block) => block.ty.route,
            FeatureRepr::ProcBlock(block) => block.ty.route,
            FeatureRepr::TargetInput { ty, .. } => *ty,
        }
    }
    pub fn feature(&self) -> FeaturePtr {
        match self {
            FeatureRepr::Value { feature, .. } => *feature,
            FeatureRepr::Expr(expr) => expr.feature,
            FeatureRepr::LazyBlock(block) => block.feature,
            FeatureRepr::FuncBlock(block) => block.feature,
            FeatureRepr::ProcBlock(block) => block.feature,
            FeatureRepr::TargetInput { feature, .. } => *feature,
        }
    }

    pub fn file(&self) -> FilePtr {
        match self {
            FeatureRepr::Value { file, .. } => *file,
            FeatureRepr::Expr(expr) => expr.expr.file,
            FeatureRepr::LazyBlock(block) => block.file,
            FeatureRepr::FuncBlock(block) => block.file,
            FeatureRepr::ProcBlock(block) => block.file,
            FeatureRepr::TargetInput { .. } => todo!(),
        }
    }

    pub fn text_range(&self) -> TextRange {
        match self {
            FeatureRepr::Value { range, .. } => *range,
            FeatureRepr::Expr(expr) => expr.expr.range,
            FeatureRepr::LazyBlock(block) => block.range,
            FeatureRepr::FuncBlock(block) => block.range,
            FeatureRepr::ProcBlock(block) => block.range,
            FeatureRepr::TargetInput { .. } => todo!(),
        }
    }

    pub fn from_defn(
        db: &dyn FeatureGenQueryGroup,
        opt_this: Option<FeatureRepr>,
        defn_repr: &DefinitionRepr,
        feature_interner: &FeatureInterner,
    ) -> Self {
        let result = match defn_repr {
            DefinitionRepr::LazyExpr { expr } => FeatureRepr::Expr(FeatureExpr::new(
                db,
                opt_this,
                expr.clone(),
                &[],
                None,
                feature_interner,
            )),
            DefinitionRepr::LazyBlock { stmts, ty } => FeatureRepr::LazyBlock(
                FeatureLazyBlock::new(db, opt_this, stmts, &[], None, feature_interner, *ty),
            ),
            DefinitionRepr::FuncBlock {
                stmts,
                file,
                range,
                route,
                output_ty,
            } => FeatureRepr::FuncBlock(Arc::new(FeatureFuncBlock {
                file: *file,
                range: *range,
                eval_id: Default::default(),
                stmts: stmts.clone(),
                instruction_sheet: {
                    let target_entrance = db.comptime().target_entrance();
                    new_func_instruction_sheet(
                        db.upcast(),
                        [].into_iter(),
                        stmts,
                        opt_this.is_some(),
                    )
                },
                feature: {
                    feature_interner.intern(match opt_this {
                        Some(ref this) => Feature::FieldAccess {
                            this: this.feature(),
                            field_ident: route.ident().custom(),
                        },
                        None => Feature::EntityFeature {
                            route: *route,
                            uid: db.comptime().entity_uid(*route),
                        },
                    })
                },
                opt_this,
                ty: *output_ty,
                opt_linkage: { db.comptime().feature_eager_block_linkage(*route) },
            })),
            DefinitionRepr::ProcBlock {
                stmts,
                file,
                range,
                ty,
            } => {
                todo!()
            }
        };
        result
    }
}

impl<'eval> From<Arc<FeatureExpr>> for FeatureRepr {
    fn from(expr: Arc<FeatureExpr>) -> Self {
        Self::Expr(expr)
    }
}

impl<'eval> From<Arc<FeatureLazyBlock>> for FeatureRepr {
    fn from(block: Arc<FeatureLazyBlock>) -> Self {
        Self::LazyBlock(block)
    }
}