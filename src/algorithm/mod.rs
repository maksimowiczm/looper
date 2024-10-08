use crate::message_bus::MessageBus;

pub mod builder;

#[derive(Debug, Clone)]
pub struct AlgorithmEvent {}

#[derive(Debug, Clone)]
pub struct AlgorithmParameters {
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
