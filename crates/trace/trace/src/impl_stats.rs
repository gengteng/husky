use crate::*;
use husky_compile_time::*;
use husky_trace_protocol::TraceStats;
use husky_word::RootIdentifier;
use vm::{__Register, __RegisterDataKind, __VMResult, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};

impl<'eval> TraceVariant<'eval> {
    pub fn opt_stats(&self, runtime: &dyn EvalFeature) -> __VMResult<Option<TraceStats>> {
        match self {
            TraceVariant::Main(repr) => feature_repr_opt_stats(runtime, repr, None),
            TraceVariant::Module { route, file, range } => Ok(None),
            TraceVariant::EntityFeature { repr, .. } => feature_repr_opt_stats(runtime, repr, None),
            TraceVariant::FeatureStmt(_) => todo!(),
            TraceVariant::FeatureBranch(_) => todo!(),
            TraceVariant::FeatureExpr(_) => todo!(),
            TraceVariant::FeatureCallArgument { name, argument } => todo!(),
            TraceVariant::FuncStmt { stmt, history } => todo!(),
            TraceVariant::ProcStmt { stmt, history } => todo!(),
            TraceVariant::ProcBranch {
                stmt,
                branch,
                opt_vm_branch,
                branch_idx,
                history,
            } => todo!(),
            TraceVariant::FuncBranch {
                stmt,
                branch,
                opt_vm_branch,
                branch_idx,
                history,
            } => todo!(),
            TraceVariant::LoopFrame {
                loop_stmt,
                body_instruction_sheet,
                body_stmts,
                loop_frame_data,
            } => todo!(),
            TraceVariant::EagerExpr { expr, history } => todo!(),
            TraceVariant::EagerCallArgument {
                name,
                argument,
                history,
            } => todo!(),
            TraceVariant::CallHead { entity, tokens } => todo!(),
        }
    }
}

const MAX_SAMPING_SIZE: usize = 1000;

fn feature_repr_opt_stats(
    db: &dyn EvalFeature,
    repr: &FeatureRepr,
    opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
) -> __VMResult<Option<TraceStats>> {
    let comptime = db.comptime();
    let target_output_ty = comptime.target_output_ty().unwrap();
    // todo check this could cause some problem
    if !comptime.is_implicitly_castable(repr.ty(), target_output_ty) {
        return Ok(None);
    }
    let mut samples = 0;
    let mut arrivals = 0;
    let mut nulls = 0;
    let mut trues = 0;
    let mut falses = 0;
    let convert_register_to_label = {
        let target_output_ty_intrinsic = target_output_ty.intrinsic();
        if target_output_ty_intrinsic == RootIdentifier::I32.into() {
            convert_i32_register_to_label
        } else {
            let target_output_ty_intrinsic_decl =
                comptime.ty_decl(target_output_ty_intrinsic).unwrap();
            use entity_kind::TyKind;
            match target_output_ty_intrinsic_decl.ty_kind {
                TyKind::Enum => convert_enum_register_to_label,
                TyKind::Record => todo!(),
                TyKind::Struct => todo!(),
                TyKind::Primitive => todo!(),
                TyKind::Vec => todo!(),
                TyKind::Slice => todo!(),
                TyKind::CyclicSlice => todo!(),
                TyKind::Array => todo!(),
                TyKind::Tuple => todo!(),
                TyKind::Mor => todo!(),
                TyKind::Fp => todo!(),
                TyKind::AssociatedAny => todo!(),
                TyKind::ThisAny => todo!(),
                TyKind::TargetOutputAny => todo!(),
                TyKind::SpatialPlaceholderAny => todo!(),
                TyKind::BoxAny => todo!(),
                TyKind::HigherKind => todo!(),
                TyKind::Ref => todo!(),
                TyKind::Option => todo!(),
            }
        }
    };
    for labeled_data in db.session().dev().each_labeled_data() {
        samples += 1;
        let sample_id = labeled_data.sample_id;
        if !db
            .eval_opt_arrival_indicator(opt_arrival_indicator, sample_id)
            .map_err(|_| todo!())?
        {
            continue;
        }
        arrivals += 1;
        let value = db
            .eval_feature_repr_cached(repr, sample_id)
            .map_err(|_| todo!())?;
        if let Some(prediction) = convert_register_to_label(&value) {
            match prediction == labeled_data.label {
                true => trues += 1,
                false => falses += 1,
            }
        } else {
            nulls += 1
        }
    }
    Ok(Some(TraceStats::Classification {
        samples,
        arrivals,
        nulls,
        trues,
        falses,
    }))
}

fn convert_enum_register_to_label<'eval>(value: &__Register<'eval>) -> Option<Label> {
    match value.data_kind() {
        __RegisterDataKind::PrimitiveValue => todo!(),
        __RegisterDataKind::Box => todo!(),
        __RegisterDataKind::EvalRef => Some(Label(
            value
                .downcast_temp_ref::<__VirtualEnum>(&__VIRTUAL_ENUM_VTABLE)
                .kind_idx,
        )),
        __RegisterDataKind::TempRef => todo!(),
        __RegisterDataKind::TempMut => todo!(),
        __RegisterDataKind::Moved => todo!(),
        __RegisterDataKind::Undefined => None,
        __RegisterDataKind::Unreturned => todo!(),
    }
}

fn convert_i32_register_to_label<'eval>(value: &__Register<'eval>) -> Option<Label> {
    match value.data_kind() {
        __RegisterDataKind::PrimitiveValue => todo!(),
        __RegisterDataKind::Box => todo!(),
        __RegisterDataKind::EvalRef => Some(Label(value.downcast_i32())),
        __RegisterDataKind::TempRef => todo!(),
        __RegisterDataKind::TempMut => todo!(),
        __RegisterDataKind::Moved => todo!(),
        __RegisterDataKind::Undefined => None,
        __RegisterDataKind::Unreturned => todo!(),
    }
}