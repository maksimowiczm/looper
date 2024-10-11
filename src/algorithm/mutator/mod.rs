use crate::algorithm::population::{Individual, Population};
use itertools::Itertools;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod best_mutator;
mod current_mutator;
mod random_mutator;

pub use best_mutator::BestMutator;
pub use current_mutator::CurrentMutator;
pub use random_mutator::RandomMutator;

pub trait Mutate {
    fn mutate(&self, factor: f64, current: &Individual, population: &Population) -> Individual;
}

fn difference(
    start: &Individual,
    how_many: usize,
    factor: f64,
    mut population: Vec<&Individual>,
) -> Individual {
    population.shuffle(&mut thread_rng());

    population
        .into_iter()
        .chunks(2)
        .into_iter()
        .take(how_many)
        .fold(start.clone(), |current, next| {
            let next = next.collect::<Vec<&Individual>>();
            assert_eq!(
                next.len(),
                2,
                "You can't have more differences than half the population"
            );

            let lhs = next[0];
            let rhs = next[1];

            let difference = (lhs.clone() - rhs.clone()) * factor;
            current + difference
        })
        .clone()
}
