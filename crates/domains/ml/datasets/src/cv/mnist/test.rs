use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MnistTestLoader {
    permutation: Arc<Vec<u32>>,
}

impl MnistTestLoader {
    pub fn new(permutation: Arc<Vec<u32>>) -> Self {
        Self { permutation }
    }
}

impl<'eval> LoadSample<'eval> for MnistTestLoader {
    fn len(&self) -> usize {
        10000
    }

    fn load<'a>(&'a mut self, idx: usize) -> LabeledData<'eval> {
        todo!()
    }
}