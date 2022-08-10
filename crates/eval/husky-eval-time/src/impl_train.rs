use crate::*;
use husky_entity_semantics::StoreEntityRoute;
use husky_feature_gen::{FeatureArrivalIndicator, FeatureExpr, TrainModel};
use std::time::Instant;
use upcast::Upcast;
use vm::{InterpreterQueryGroup, VMConfig, __Register, __VMResult};
use vm::{__RegistrableSafe, __VirtualEnum};

impl TrainModel for HuskyRuntime {
    fn train(
        &self,
        model: vm::__ModelLinkage,
        opt_arrival_indicator: Option<&Arc<FeatureArrivalIndicator>>,
        opds: &[Arc<FeatureExpr>],
    ) -> vm::__VMResult<__Register<'static>> {
        const MAX_SAMPLE_LEN: usize = 1000;
        let session = self.session();
        let dev_division = session.dev();
        let mut label_statics_map: HashMap<i32, HashMap<Label, usize>> = Default::default();
        let now = Instant::now();
        let mut training_data: Vec<(Vec<__Register>, __Register)> = Vec::new();
        for labeled_data in dev_division.each_labeled_data() {
            let sample_id = labeled_data.sample_id;
            if !self.eval_opt_arrival_indicator(opt_arrival_indicator, sample_id)? {
                continue;
            }
            if training_data.len() >= MAX_SAMPLE_LEN {
                break;
            }
            let values: Vec<__Register> = opds
                .iter()
                .map(|opd| self.eval_feature_expr(opd, sample_id))
                .collect::<__VMResult<Vec<_>>>()
                .map_err(|e| {
                    todo!()
                    // (sample_id, e)
                })?;
            training_data.push((
                values,
                __VirtualEnum {
                    kind_idx: labeled_data.label.0,
                }
                .to_register(),
            ))
        }
        let train_result = model.train_dyn(training_data);
        println!(
            "{} milliseconds elapsed for evaluating first 1000 in naive train",
            now.elapsed().as_millis(),
        );
        train_result
    }
}