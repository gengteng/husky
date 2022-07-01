mod impl_opn;
mod xml;

use vm::Linkage;
pub use xml::*;

use husky_entity_route_syntax::EntityRouteKind;
use husky_entity_route_syntax::{EntityRoutePtr, RangedEntityRoute};
use husky_entity_semantics::*;
use husky_lazy_semantics::*;
use std::sync::Arc;
use vm::{Binding, EvalResult, EvalValue, InstructionSheet, SpecificRoutineLinkage};
use word::RootIdentifier;

use crate::{eval_id::FeatureEvalId, *};

#[derive(Debug, Clone)]
pub struct FeatureExpr {
    pub variant: FeatureLazyExprVariant,
    pub feature: FeaturePtr,
    pub eval_id: FeatureEvalId,
    pub expr: Arc<LazyExpr>,
}

impl std::hash::Hash for FeatureExpr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.eval_id.hash(state)
    }
}

impl<'eval> PartialEq for FeatureExpr {
    fn eq(&self, other: &Self) -> bool {
        self.eval_id == other.eval_id
    }
}

impl<'eval> Eq for FeatureExpr {}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum FeatureLazyExprVariant {
    PrimitiveLiteral(CopyableValue),
    EnumKindLiteral {
        entity_route: EntityRoutePtr,
        uid: EntityUid,
    },
    PrimitiveBinaryOpr {
        opr: PureBinaryOpr,
        lopd: Arc<FeatureExpr>,
        ropd: Arc<FeatureExpr>,
    },
    Variable {
        varname: CustomIdentifier,
        value: Arc<FeatureExpr>,
    },
    ThisValue {
        repr: FeatureRepr,
    },
    StructOriginalFieldAccess {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        field_idx: usize,
        field_binding: Binding,
        opt_linkage: Option<SpecificRoutineLinkage>,
    },
    RecordOriginalFieldAccess {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    StructDerivedLazyFieldAccess {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    RecordDerivedFieldAccess {
        this: FeatureRepr,
        field_ident: RangedCustomIdentifier,
        repr: FeatureRepr,
    },
    ElementAccess {
        opds: Vec<Arc<FeatureExpr>>,
        linkage: SpecificRoutineLinkage,
    },
    ModelCall {
        opds: Vec<Arc<FeatureExpr>>,
        has_this: bool,
        model_defn: Arc<EntityDefn>,
        internal: EvalResult,
    },
    RoutineCall {
        opds: Vec<Arc<FeatureExpr>>,
        has_this: bool,
        opt_instruction_sheet: Option<Arc<InstructionSheet>>,
        opt_linkage: Option<Linkage>,
        routine_defn: Arc<EntityDefn>,
    },
    EntityFeature {
        entity_route: EntityRoutePtr,
        repr: FeatureRepr,
    },
    EvalInput,
    NewRecord {
        ty: RangedEntityRoute,
        entity: Arc<EntityDefn>,
        opds: Vec<Arc<FeatureExpr>>,
    },
}

impl FeatureExpr {
    pub fn new(
        db: &(dyn FeatureGenQueryGroup),
        this: Option<FeatureRepr>,
        expr: Arc<LazyExpr>,
        symbols: &[FeatureSymbol],
        features: &FeatureInterner,
    ) -> Arc<Self> {
        FeatureExprBuilder {
            db,
            symbols,
            features,
            opt_this: this,
        }
        .new_expr(expr)
    }
}

struct FeatureExprBuilder<'a> {
    db: &'a dyn FeatureGenQueryGroup,
    symbols: &'a [FeatureSymbol],
    features: &'a FeatureInterner,
    opt_this: Option<FeatureRepr>,
}

impl<'a> FeatureExprBuilder<'a> {
    fn new_expr(&self, expr: Arc<LazyExpr>) -> Arc<FeatureExpr> {
        let (kind, feature) = match expr.variant {
            LazyExprVariant::Variable { varname, .. } => self
                .symbols
                .iter()
                .rev()
                .find_map(|symbol| {
                    if symbol.varname == varname {
                        Some((
                            FeatureLazyExprVariant::Variable {
                                varname,
                                value: symbol.value.clone(),
                            },
                            symbol.feature,
                        ))
                    } else {
                        None
                    }
                })
                .unwrap(),
            LazyExprVariant::EntityRoute { .. } => todo!(),
            LazyExprVariant::PrimitiveLiteral(value) => (
                FeatureLazyExprVariant::PrimitiveLiteral(value),
                self.features.intern(Feature::PrimitiveLiteral(value)),
            ),
            LazyExprVariant::Bracketed(ref bracketed_expr) => {
                return self.new_expr(bracketed_expr.clone())
            }
            LazyExprVariant::Opn { opn_kind, ref opds } => self.compile_opn(opn_kind, opds, &expr),
            LazyExprVariant::Lambda(_, _) => todo!(),
            LazyExprVariant::EnumLiteral { entity_route } => (
                FeatureLazyExprVariant::EnumKindLiteral {
                    entity_route,
                    uid: self.db.compile_time().entity_uid(entity_route),
                },
                self.features.intern(Feature::EnumLiteral(entity_route)),
            ),
            LazyExprVariant::ThisValue { .. } => (
                FeatureLazyExprVariant::ThisValue {
                    repr: self.opt_this.as_ref().unwrap().clone(),
                },
                self.opt_this.as_ref().unwrap().feature(),
            ),
            LazyExprVariant::ThisField {
                field_ident,
                this_ty,
                this_binding,
                field_binding,
            } => {
                let this_repr = self.opt_this.clone().unwrap();
                self.compile_field_access(field_ident, this_repr, field_binding)
            }
            LazyExprVariant::EntityFeature { entity_route } => match entity_route.kind {
                EntityRouteKind::Root { .. } | EntityRouteKind::Package { .. } => panic!(),
                EntityRouteKind::Child { .. } => {
                    let uid = self.db.compile_time().entity_uid(entity_route);
                    let feature = self.features.intern(Feature::EntityFeature {
                        route: entity_route,
                        uid,
                    });
                    let kind = FeatureLazyExprVariant::EntityFeature {
                        entity_route,
                        repr: self.db.entity_feature_repr(entity_route),
                    };
                    (kind, feature)
                }
                EntityRouteKind::Input { main } => {
                    let feature = self.features.intern(Feature::Input);
                    let kind = FeatureLazyExprVariant::EvalInput;
                    (kind, feature)
                }
                EntityRouteKind::Generic { ident, .. } => todo!(),
                EntityRouteKind::ThisType => todo!(),
                EntityRouteKind::TypeAsTraitMember { .. } => todo!(),
            },
        };
        Arc::new(FeatureExpr {
            variant: kind,
            feature,
            eval_id: Default::default(),
            expr,
        })
    }
}