use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::Population;
use crate::algorithm::Individual;

pub(super) struct DifferentialEvolution<'a> {
    pub mutator: &'a dyn Mutate,
    pub crossover_rate: f64,
}

impl DifferentialEvolution<'_> {
    pub fn evolve(&self, population: Population) -> Population {
        todo!()
    }

    pub fn crossover(parent: &Individual, mutant: Individual) -> Individual {
        todo!()
    }
}
