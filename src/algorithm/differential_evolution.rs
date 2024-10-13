use crate::algorithm::evaluator::Evaluator;
use crate::algorithm::individual::Individual;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::Population;
use crate::algorithm::Domain;
use rand::Rng;

pub(super) struct DifferentialEvolution<'a> {
    pub mutator: &'a dyn Mutate,
    pub evaluator: Evaluator,
    pub crossover_probability: f64,
}

impl DifferentialEvolution<'_> {
    pub fn evolve(&self, mutation_factor: f64, population: &mut Population, domain: &[Domain]) {
        let mutants = population
            .iter()
            .map(|individual| {
                Self::mutate(
                    self.mutator,
                    mutation_factor,
                    individual,
                    population,
                    domain,
                )
            })
            .collect::<Vec<_>>();

        population
            .iter_mut()
            .zip(mutants)
            .for_each(|(individual, mutant)| {
                let new = Self::crossover(individual, mutant, self.crossover_probability);

                // if the new individual is better than the current one, replace it
                if (self.evaluator)(individual) > (self.evaluator)(&new) {
                    *individual = new;
                }
            })
    }

    fn mutate(
        mutator: &dyn Mutate,
        mutation_factor: f64,
        individual: &Individual,
        population: &Population,
        domain: &[Domain],
    ) -> Individual {
        let mutant = mutator.mutate(mutation_factor, individual, population);

        domain
            .iter()
            .zip(mutant.iter())
            .map(|(&d, &mutant)| {
                if mutant < d.0 {
                    d.0
                } else if mutant > d.1 {
                    d.1
                } else {
                    mutant
                }
            })
            .collect()
    }

    fn crossover(
        parent: &Individual,
        mutant: Individual,
        crossover_probability: f64,
    ) -> Individual {
        // ensure something crosses
        let j = rand::thread_rng().gen_range(0..parent.len());

        parent
            .iter()
            .zip(mutant.iter())
            .enumerate()
            .map(|(i, (parent_gene, mutant_gene))| {
                let rng = rand::thread_rng().gen_range(0.0..1.);

                if i == j || rng < crossover_probability {
                    *mutant_gene
                } else {
                    *parent_gene
                }
            })
            .collect()
    }
}
