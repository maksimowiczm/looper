use crate::algorithm::mutator;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::{Individual, Population};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub struct RandomMutator {
    how_many: usize,
}

impl Mutate for RandomMutator {
    fn mutate(&self, factor: f64, _: &Individual, population: &Population) -> Individual {
        let random_individual = population
            .choose(&mut thread_rng())
            .expect("Population should not be empty");

        mutator::difference(
            random_individual,
            self.how_many,
            factor,
            population
                .iter()
                .filter(|x| *x != random_individual)
                .collect(),
        )
    }
}
