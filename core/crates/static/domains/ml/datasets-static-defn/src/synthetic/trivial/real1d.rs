use crate::{synthetic::SimpleSyntheticDataset, *};
use entity_kind::RoutineKind;
use husky_tracer_protocol::SampleId;
use liason::OutputLiason;
use std::sync::Arc;
use vm::{routine_linkage, EvalValue, OwnedValue, RoutineLinkage, TempValue};
use xrng::XRng;

pub const REAL_1D_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "real1d",
    items: &[DATASET1_MODULE_DEFN, DATASET2_SCOPE_DATA],
    variant: EntityStaticDefnVariant::Module,
    dev_src: dev_utils::static_dev_src!(),
};

pub const DATASET1_MODULE_DEFN: &EntityStaticDefn = &EntityStaticDefn {
    name: "dataset1",
    items: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: &[],
        output_ty: "Dataset<f32, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: routine_linkage!(|_| Ok(TempValue::OwnedEval(OwnedValue::new(dataset1()))), 0),
        routine_kind: RoutineKind::Normal,
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub const DATASET2_SCOPE_DATA: &EntityStaticDefn = &EntityStaticDefn {
    name: "dataset2",
    items: &[],
    variant: EntityStaticDefnVariant::Routine {
        generic_parameters: &[],
        parameters: &[],
        output_ty: "Dataset<f32, i32>",
        output_liason: OutputLiason::Transfer,
        linkage: routine_linkage!(|_| Ok(TempValue::OwnedEval(OwnedValue::new(dataset2()))), 0),
        routine_kind: RoutineKind::Normal,
    },
    dev_src: dev_utils::static_dev_src!(),
};

pub fn gen_sample1<'eval>(seed: u64, sample_id: SampleId) -> LabeledData<'eval> {
    let mut xrng = XRng::new(((seed + (sample_id.0 as u64)) >> 32) & ((sample_id.0 as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: EvalValue::Copyable(1.0f32.into()),
            label: 1.into(),
            sample_id: sample_id,
        }
    } else {
        LabeledData {
            input: EvalValue::Copyable((-1.0f32).into()),
            label: 1.into(),
            sample_id: sample_id,
        }
    }
}

pub fn gen_sample2<'eval>(seed: u64, sample_id: SampleId) -> LabeledData<'eval> {
    let mut xrng = XRng::new(((seed + (sample_id.0 as u64)) >> 32) & ((sample_id.0 as u64) << 32));
    if xrng.with_probability(0.5) {
        LabeledData {
            input: EvalValue::Copyable(1.0f32.into()),
            label: 1.into(),
            sample_id: sample_id,
        }
    } else {
        LabeledData {
            input: EvalValue::Copyable((-1.0f32).into()),
            label: 1.into(),
            sample_id: sample_id,
        }
    }
}

pub fn dataset1<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1223418012u64, gen_sample1))
}

pub fn dataset2<'eval>() -> Dataset<'eval> {
    Dataset::new(SimpleSyntheticDataset::new(1213148012u64, gen_sample2))
}