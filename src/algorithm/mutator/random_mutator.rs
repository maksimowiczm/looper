use crate::algorithm::individual::Individual;
use crate::algorithm::mutator;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::Population;
use rand::seq::SliceRandom;
use rand::thread_rng;

#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct RandomMutator {
    pub(super) size: usize,
}

impl Mutate for RandomMutator {
    fn mutate(&self, factor: f64, _: &Individual, population: &Population) -> Individual {
        let random_individual = population
            .choose(&mut thread_rng())
            .expect("Population should not be empty");

        mutator::difference(
            random_individual,
            self.size,
            factor,
            population
                .iter()
                .filter(|x| *x != random_individual)
                .collect(),
        )
    }

    fn vector_size(&self) -> usize {
        self.size
    }
}
