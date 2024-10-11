use crate::algorithm::mutator;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::{Individual, Population};

pub struct CurrentMutator {
    how_many: usize,
}

impl Mutate for CurrentMutator {
    fn mutate(&self, factor: f64, current: &Individual, population: &Population) -> Individual {
        mutator::difference(
            current,
            self.how_many,
            factor,
            population.iter().filter(|x| *x != current).collect(),
        )
    }
}
