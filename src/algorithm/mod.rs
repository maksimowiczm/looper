use crate::algorithm::differential_evolution::DifferentialEvolution;
use crate::algorithm::evaluator::Evaluate;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::Individual;
use crate::message_bus::MessageBus;

pub mod builder;
mod differential_evolution;
pub mod evaluator;
pub mod mutator;
pub mod population;

#[derive(Debug, Clone)]
pub struct AlgorithmEvent {}

pub struct AlgorithmParameters {
    pub iterations: usize,
    pub population_size: usize,
    //
    // Something about individual. How many variables it has? What is the domain? Hard coupled with evaluator.
    //
    pub evaluator: Box<dyn Evaluate>, // might as well replace this with a closure
    pub mutator: Box<dyn Mutate>, // Couple it with evaluator? They both have to know how many variables are there.
    pub mutation_factor: f64,
    pub crossover_probability: f64,
}

pub struct Algorithm {
    message_bus: MessageBus<AlgorithmEvent>,
    algorithm_parameters: AlgorithmParameters,
}

impl Algorithm {
    pub async fn run(&self) {
        let params = &self.algorithm_parameters;

        let differential_evolution = DifferentialEvolution {
            mutator: params.mutator.as_ref(),
            crossover_probability: params.crossover_probability,
        };

        // initialize population

        for i in 0..params.iterations {
            // evaluate population

            // send iteration event

            // depending on mutation strategy do something different

            // evolve population
        }

        // send finished event
    }
}
