use super::*;

#[derive(Prop)]
pub struct GenericI32Props<'a> {
    dimension: &'a ReadSignal<PixelDimension>,
    partitioned_samples: &'a [(PartitionDefnData, Vec<(SampleId, i32)>)],
}

#[component]
pub fn GenericI32<'a, G: Html>(scope: Scope<'a>, props: GenericI32Props<'a>) -> View<G> {
    let map = props
        .partitioned_samples
        .iter()
        .map(|(_, sample_values)| sample_values.iter())
        .flatten()
        .map(|(_, value)| value);
    let max = *map.clone().max().unwrap();
    let min = *map.min().unwrap();
    let bins: Vec<Vec<(&'a PartitionDefnData, Vec<SampleId>)>> = (min..(max + 1))
        .into_iter()
        .map(|dv| {
            props
                .partitioned_samples
                .iter()
                .map(
                    |(partition_defn_data, samples)| -> (&'a PartitionDefnData, Vec<SampleId>) {
                        (
                            partition_defn_data,
                            samples
                                .iter()
                                .filter_map(|(sample_id, value)| {
                                    if min + dv == *value {
                                        Some(*sample_id)
                                    } else {
                                        None
                                    }
                                })
                                .collect(),
                        )
                    },
                )
                .collect()
        })
        .collect();
    let bins = View::new_fragment(
        bins.into_iter()
            .enumerate()
            .map(|(i, partitioned_samples)| {
                view! {
                    scope,
                    GenericI32Bin {
                        label: min + (i as i32),
                        partitioned_samples
                    }
                }
            })
            .collect(),
    );
    view! {
        scope,
        div (
            class="GenericI32",
            style=props.dimension.cget().to_style(),
        ) {
            (bins)
        }
    }
}

#[derive(Prop)]
pub struct GenericI32BinProps<'a> {
    label: i32,
    partitioned_samples: Vec<(&'a PartitionDefnData, Vec<SampleId>)>,
}

#[component]
fn GenericI32Bin<'a, G: Html>(scope: Scope<'a>, props: GenericI32BinProps<'a>) -> View<G> {
    let mut all_samples = vec![];
    props
        .partitioned_samples
        .into_iter()
        .for_each(|(partition_defn, samples)| {
            all_samples.extend(samples.iter().map(move |sample_id| {
                view! {
                    scope,
                    GenericI32Sample {
                        partition_defn,
                        sample_id:*sample_id,
                    }
                }
            }))
        });
    let all_samples = View::new_fragment(all_samples);
    view! {
        scope,
        div (class="GenericI32Bin") {
            div (class="GenericI32BinPartitions") {
                (all_samples)
            }
            div (class="GenericI32BinLabel") {
                (props.label)
            }
        }
    }
}

#[derive(Prop)]
pub struct GenericI32SampleProps<'a> {
    partition_defn: &'a PartitionDefnData,
    sample_id: SampleId,
}

#[component]
fn GenericI32Sample<'a, G: Html>(scope: Scope<'a>, props: GenericI32SampleProps<'a>) -> View<G> {
    view! {
        scope,
        div (class="GenericI32Sample")
    }
}
