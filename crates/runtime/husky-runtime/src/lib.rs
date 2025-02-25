mod impl_necessary;
mod impl_train;
mod query;
mod variant;

pub use husky_comptime::*;
pub use husky_feature_gen::{FeatureGenQueryGroup, FeatureGenQueryGroupStorage, InternFeature};
pub use husky_instruction_gen::InstructionGenQueryGroup;
pub use query::*;

use husky_check_utils::*;
use husky_feature_eval::*;
use husky_feature_eval::{EvalFeature, Session};
use husky_file::FileQueryGroup;
use husky_print_utils::*;
use std::sync::Arc;
use variant::*;

#[salsa::database(
    husky_feature_gen::FeatureGenQueryGroupStorage,
    husky_instruction_gen::InstructionGenQueryGroupStorage,
    husky_data_viewer::HuskyDataViewerQueryGroupStorage
)]
pub struct HuskyRuntime {
    storage: salsa::Storage<HuskyRuntime>,
    comptime: HuskyComptime,
    feature_interner: husky_feature_gen::FeatureInterner,
    variant: HuskyRuntimeVariant,
    config: HuskyRuntimeConfig,
}

#[derive(Debug)]
pub struct HuskyRuntimeConfig {
    pub evaluator: EvaluatorConfig,
    pub comptime: HuskyComptimeConfig,
}

impl HuskyRuntime {
    pub fn new(
        init_comptime: impl FnOnce(&mut HuskyComptime),
        config: HuskyRuntimeConfig,
    ) -> HuskyRuntime {
        let mut comptime = HuskyComptime::new(config.comptime.clone());
        init_comptime(&mut comptime);
        let all_main_files = comptime.all_target_entrances();
        should_eq!(all_main_files.len(), 1, "config = {config:?}");
        let feature_interner = husky_feature_gen::new_feature_interner();
        let mut eval_time = Self {
            storage: Default::default(),
            variant: HuskyRuntimeVariant::None,
            comptime,
            config,
            feature_interner,
        };
        eval_time.init();
        eval_time.into()
    }

    fn init(&mut self) {
        let comptime = &self.comptime;
        let all_diagnostics = comptime.all_diagnostics();
        if all_diagnostics.len() > 0 {
            p!(all_diagnostics);
            panic!("diagnostic errors")
        }
        let package = match comptime.package(comptime.opt_target_entrance().unwrap()) {
            Ok(package) => package,
            Err(error) => {
                comptime.print_diagnostics();
                p!(error);
                panic!()
            }
        };
        self.variant = HuskyRuntimeVariant::Learning {
            session: Session::new(&package, self, &self.evaluator_config().vm).unwrap(),
        }
    }
}
