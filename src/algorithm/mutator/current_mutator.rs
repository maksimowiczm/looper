use crate::algorithm::mutator;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::{Individual, Population};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct CurrentMutator {
    pub(super) size: usize,
}

impl Mutate for CurrentMutator {
    fn mutate(&self, factor: f64, current: &Individual, population: &Population) -> Individual {
        mutator::difference(
            current,
            self.size,
            factor,
            population.iter().filter(|x| *x != current).collect(),
        )
    }

    fn vector_size(&self) -> usize {
        self.size
    }
}
