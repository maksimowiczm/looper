use crate::algorithm::differential_evolution::DifferentialEvolution;
use crate::algorithm::evaluator::Evaluator;
use crate::algorithm::mutator::Mutate;
use crate::message_bus::MessageBus;
use population::Population;

mod differential_evolution;
pub mod evaluator;
mod individual;
pub mod mutator;
mod population;

#[derive(Debug, Clone)]
pub struct AlgorithmEvent {}

pub struct AlgorithmParameters {
    pub iterations: usize,
    pub population_size: usize,
    pub evaluator: Evaluator,
    pub mutator: Box<dyn Mutate>,
    pub mutation_factor: f64,
    pub crossover_probability: f64,
    pub domain: Vec<Domain>,
}
pub type Domain = (f64, f64);

pub struct Algorithm {
    message_bus: MessageBus<AlgorithmEvent>,
    algorithm_parameters: AlgorithmParameters,
}

impl Algorithm {
    pub fn new(
        message_bus: MessageBus<AlgorithmEvent>,
        algorithm_parameters: AlgorithmParameters,
    ) -> Self {
        Algorithm {
            message_bus,
            algorithm_parameters,
        }
    }

    pub async fn run(&self) {
        let params = &self.algorithm_parameters;

        let differential_evolution = DifferentialEvolution {
            mutator: params.mutator.as_ref(),
            crossover_probability: params.crossover_probability,
        };

        let mut population =
            Population::random(&params.domain, params.population_size, params.evaluator)
                .expect("Population should not be empty");

        for i in 0..params.iterations {
            // evaluate population

            // send iteration event

            // depending on mutation strategy do something different

            // evolve population
        }

        // send finished event
    }
}
