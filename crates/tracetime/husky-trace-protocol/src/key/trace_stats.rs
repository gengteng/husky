use smallvec::SmallVec;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct TraceStatsKey {
    pub trace_id: TraceId,
    pub partitions: Partitions,
}

const PARTITION_SMALL_VEC_SIZE: usize = 4;
const NCOL_TOTAL: u32 = 7;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize, Hash)]
pub struct Partitions {
    nondefaults: SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>,
    default_partition_ncol: u32,
}

impl Default for Partitions {
    fn default() -> Self {
        Self {
            nondefaults: Default::default(),
            default_partition_ncol: NCOL_TOTAL,
        }
    }
}

#[test]
fn test_partition_idx() {
    let mut partitions = Partitions::default();
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(0)), None);
    assert_eq!(partitions.partition_idx(Label(0)), 0);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(1)), None);
    assert_eq!(partitions.partition_idx(Label(1)), 0);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(5)), None);
    assert_eq!(partitions.partition_idx(Label(5)), 0);
    assert!(!partitions.is_nondefault(Label(0)));
    assert!(!partitions.is_nondefault(Label(5)));
    partitions.add_partition(
        0,
        PartitionDefnData {
            ncol: 3,
            variant: PartitionDefnDataVariant::Label(Label(0)),
        },
    );
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(0)), Some(0));
    assert_eq!(partitions.partition_idx(Label(0)), 0);
    println!("partitions = {:?}", partitions);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(1)), None);
    assert_eq!(partitions.partition_idx(Label(1)), 1);
    assert_eq!(partitions.opt_nondefault_partition_idx(Label(5)), None);
    assert_eq!(partitions.partition_idx(Label(5)), 1);
    assert!(partitions.is_nondefault(Label(0)));
    assert!(!partitions.is_nondefault(Label(5)));
}

impl From<SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>> for Partitions {
    fn from(nondefaults: SmallVec<[PartitionDefnData; PARTITION_SMALL_VEC_SIZE]>) -> Self {
        let nodefault_ncol_total: u32 = nondefaults
            .iter()
            .map(|p| {
                assert!(p.ncol > 0);
                p.ncol
            })
            .sum();
        let default_partition_ncol: u32 = NCOL_TOTAL - nodefault_ncol_total;
        assert!(default_partition_ncol > 0);
        Self {
            nondefaults,
            default_partition_ncol,
        }
    }
}

impl Partitions {
    pub fn add_partition(&mut self, idx: usize, new_partition: PartitionDefnData) {
        assert_ne!(new_partition.variant, PartitionDefnDataVariant::Default);
        self.default_partition_ncol -= new_partition.ncol;
        assert!(self.default_partition_ncol > 0);
        assert!(new_partition.ncol > 0);
        self.nondefaults.insert(idx, new_partition)
    }

    pub fn partition_idx(&self, label: Label) -> usize {
        self.opt_nondefault_partition_idx(label)
            .unwrap_or(self.nondefaults.len())
    }
    pub fn partition_ncol(&self, partition_idx: usize) -> u32 {
        self.nondefaults
            .get(partition_idx)
            .map(|d| d.ncol)
            .unwrap_or(self.default_partition_ncol)
    }

    pub fn opt_nondefault_partition_idx(&self, label: Label) -> Option<usize> {
        self.nondefaults
            .iter()
            .position(|partition| partition.contains(label))
    }

    pub fn is_nondefault(&self, label: Label) -> bool {
        self.opt_nondefault_partition_idx(label).is_some()
    }

    pub fn init_partition_values<T>(&self) -> Vec<(PartitionDefnData, T)>
    where
        T: Default,
    {
        (0..self.total_len())
            .into_iter()
            .map(|i| (self.defn_data(i), Default::default()))
            .collect()
    }

    fn defn_data(&self, i: usize) -> PartitionDefnData {
        if i < self.nondefaults.len() {
            self.nondefaults[i].clone()
        } else {
            self.default_partition_defn_data()
        }
    }

    fn default_partition_defn_data(&self) -> PartitionDefnData {
        PartitionDefnData {
            ncol: self.default_partition_ncol,
            variant: PartitionDefnDataVariant::Default,
        }
    }

    pub fn total_len(&self) -> usize {
        self.nondefaults.len() + 1
    }
}

// impl std::ops::Deref for Partitions {
//     type Target = [PartitionDefnData];

//     fn deref(&self) -> &Self::Target {
//         &self.nondefaults
//     }
// }
