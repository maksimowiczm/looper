use crate::algorithm::mutator::Mutator;
use crate::message_bus::MessageBus;

pub mod builder;
mod mutator;
mod population;

#[derive(Debug, Clone)]
pub struct AlgorithmEvent {}

pub struct AlgorithmParameters {
    pub mutator: Mutator,
}

pub struct Algorithm {
    message_bus: MessageBus<AlgorithmEvent>,
    algorithm_parameters: AlgorithmParameters,
}

impl Algorithm {
    pub async fn run(&self) {
        let _ = self.message_bus.send(AlgorithmEvent {}).await;
    }
}
