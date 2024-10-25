use crate::message_bus::MessageBus;
use crate::algorithm::differential_evolution::DifferentialEvolution;
use crate::algorithm::evaluator::Evaluator;
use crate::algorithm::individual::Individual;
use crate::algorithm::mutator::Mutate;
use crate::algorithm::population::Population;

mod differential_evolution;
pub mod evaluator;
pub mod individual;
pub mod mutator;
pub mod population;

#[derive(Clone)]
pub enum AlgorithmEvent {
    Iteration(usize, Vec<Individual>),
    Finished,
}

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

pub struct Algorithm<'a> {
    message_bus: &'a MessageBus<AlgorithmEvent>,
    algorithm_parameters: AlgorithmParameters,
}

impl<'a> Algorithm<'a> {
    pub fn new(
        message_bus: &'a MessageBus<AlgorithmEvent>,
        algorithm_parameters: AlgorithmParameters,
    ) -> Self {
        Algorithm {
            message_bus,
            algorithm_parameters,
        }
    }

    pub fn run(&self) {
        let params = &self.algorithm_parameters;

        let differential_evolution = DifferentialEvolution {
            mutator: params.mutator.as_ref(),
            crossover_probability: params.crossover_probability,
            evaluator: params.evaluator,
        };

        let mut population =
            Population::random(&params.domain, params.population_size, params.evaluator)
                .expect("Population should not be empty");

        for i in 0..params.iterations {
            self.notify(AlgorithmEvent::Iteration(i, population.as_ref().clone()));
            differential_evolution.evolve(params.mutation_factor, &mut population, &params.domain);
        }

        self.notify(AlgorithmEvent::Finished);
    }

    fn notify(&self, event: AlgorithmEvent) {
        if let Some(sender) = self.message_bus.as_ref() {
            let _ = sender.send(event);
        }
    }
}
