use crate::*;
use husky_comptime::{utils::__RegisterDowncastResult, *};
use husky_print_utils::msg_once;
use husky_trace_protocol::TraceStats;
use husky_vm::{__Register, __RegisterDataKind, __VMResult, __VirtualEnum, __VIRTUAL_ENUM_VTABLE};
use husky_word::RootIdentifier;

impl<'eval> TraceVariant<'eval> {
    pub fn opt_stats(
        &self,
        runtime: &dyn EvalFeature,
        partitions: &Partitions,
    ) -> __VMResult<Option<TraceStats>> {
        match self {
            TraceVariant::Main(repr) => feature_repr_opt_stats(runtime, partitions, repr, None),
            TraceVariant::Module { route, file, range } => Ok(None),
            TraceVariant::EntityFeature { repr, .. } => {
                feature_repr_opt_stats(runtime, partitions, repr, None)
            }
            TraceVariant::FeatureStmt(stmt) => feature_stmt_opt_stats(runtime, partitions, stmt),
            TraceVariant::FeatureBranch(branch) => {
                feature_branch_opt_stats(runtime, partitions, branch)
            }
            TraceVariant::FeatureExpr(expr) => feature_expr_opt_stats(runtime, partitions, expr),
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

const MAX_SAMPING_SIZE: usize = 50000;

fn feature_repr_opt_stats<'eval>(
    db: &dyn EvalFeature<'eval>,
    partitions: &Partitions,
    repr: &FeatureRepr,
    opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
) -> __VMResult<Option<TraceStats>> {
    feature_opt_stats(
        db,
        partitions,
        repr.ty(),
        |sample_id| db.eval_feature_repr_cached(repr, sample_id),
        opt_arrival_indicator,
    )
}

fn feature_stmt_opt_stats<'eval>(
    db: &dyn EvalFeature<'eval>,
    partitions: &Partitions,
    stmt: &FeatureLazyStmt,
) -> __VMResult<Option<TraceStats>> {
    match stmt.variant {
        FeatureLazyStmtVariant::Init { .. } | FeatureLazyStmtVariant::Assert { .. } => Ok(None),
        FeatureLazyStmtVariant::Require { return_context, .. } => feature_opt_stats(
            db,
            partitions,
            return_context.return_ty.route,
            |sample_id| db.eval_feature_stmt(stmt, sample_id),
            stmt.opt_arrival_indicator.as_ref(),
        ),
        FeatureLazyStmtVariant::Return { ref result } => {
            feature_expr_opt_stats(db, partitions, result)
        }
        FeatureLazyStmtVariant::ReturnXml { ref result } => todo!(),
        FeatureLazyStmtVariant::ConditionFlow { ref branches } => todo!(),
    }
}

fn feature_branch_opt_stats<'eval>(
    db: &dyn EvalFeature<'eval>,
    partitions: &Partitions,
    branch: &FeatureLazyBranch,
) -> __VMResult<Option<TraceStats>> {
    msg_once!("consider whether condition is satisfied");
    feature_opt_stats(
        db,
        partitions,
        branch.block.ty.route,
        |sample_id| -> __VMResult<__Register<'eval>> {
            match branch.variant {
                FeatureLazyBranchVariant::If { ref condition } => {
                    if !db.eval_feature_expr(condition, sample_id)?.to_bool() {
                        return Ok(__Register::new_unreturned());
                    }
                }
                FeatureLazyBranchVariant::Elif { ref condition } => {
                    if !db.eval_feature_expr(condition, sample_id)?.to_bool() {
                        return Ok(__Register::new_unreturned());
                    }
                }
                FeatureLazyBranchVariant::Else => (),
            }
            db.eval_feature_lazy_block(&branch.block, sample_id)
        },
        branch.opt_arrival_indicator.as_ref(),
    )
}

fn feature_expr_opt_stats<'eval>(
    db: &dyn EvalFeature<'eval>,
    partitions: &Partitions,
    expr: &FeatureLazyExpr,
) -> __VMResult<Option<TraceStats>> {
    feature_opt_stats(
        db,
        partitions,
        expr.expr.ty(),
        |sample_id| db.eval_feature_expr(expr, sample_id),
        expr.opt_arrival_indicator.as_ref(),
    )
}

fn feature_opt_stats<'eval>(
    db: &dyn EvalFeature,
    partitions: &Partitions,
    feature_ty: EntityRoutePtr,
    compute_value: impl Fn(SampleId) -> __VMResult<__Register<'eval>>,
    opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
) -> __VMResult<Option<TraceStats>> {
    let comptime = db.comptime();
    let target_output_ty = comptime.target_output_ty().unwrap();
    // todo check this could cause some problem
    if !comptime.is_implicitly_castable(feature_ty, target_output_ty) {
        return Ok(None);
    }
    let mut dev_samples = 0;
    let mut dev_arrivals = 0;
    let mut dev_unreturneds = 0;
    let mut dev_nones = 0;
    let mut dev_trues = 0;
    let mut dev_falses = 0;
    let mut dev_partition_noness = partitions.init_partition_values();
    let convert_register_to_label = comptime.register_to_label_converter();
    for labeled_data in db.session().dev().each_labeled_data() {
        if dev_samples >= MAX_SAMPING_SIZE {
            break;
        }
        dev_samples += 1;
        let sample_id = labeled_data.sample_id;
        if !db
            .eval_opt_arrival_indicator(opt_arrival_indicator, sample_id)
            .map_err(|_| todo!())?
        {
            continue;
        }
        dev_arrivals += 1;
        let value = compute_value(sample_id).map_err(|e| {
            p!(e);
            todo!()
        })?;
        match convert_register_to_label(&value) {
            __RegisterDowncastResult::Value(prediction) => match prediction == labeled_data.label {
                true => dev_trues += 1,
                false => dev_falses += 1,
            },
            __RegisterDowncastResult::None => {
                dev_nones += 1;
                let idx = partitions.partition_idx(labeled_data.label);
                dev_partition_noness[idx].1 += 1;
            }
            __RegisterDowncastResult::Unreturned => dev_unreturneds += 1,
        }
    }
    Ok(Some(TraceStats::Classification {
        dev_samples,
        dev_arrivals,
        dev_unreturneds,
        dev_nones,
        dev_trues,
        dev_falses,
        dev_partition_noness,
    }))
}