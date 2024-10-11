use crate::algorithm::mutator;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::{Individual, Population};

pub struct BestMutator {
    how_many: usize,
}

impl Mutate for BestMutator {
    fn mutate(&self, factor: f64, _: &Individual, population: &Population) -> Individual {
        let best = population.best();

        mutator::difference(
            best,
            self.how_many,
            factor,
            population.iter().filter(|x| *x != best).collect(),
        )
    }
}
