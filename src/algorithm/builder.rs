use crate::algorithm::{Algorithm, AlgorithmEvent, AlgorithmParameters};
use crate::message_bus::MessageBus;

pub struct AlgorithmBuilder<B, P> {
    message_bus: B,
    algorithm_parameters: P,
}

// region builder states
pub struct NoMessageBus;
pub struct WithMessageBus<M>(MessageBus<M>);

pub struct NoAlgorithmParameters;
pub struct WithAlgorithmParameters<'a>(AlgorithmParameters<'a>);
// endregion

impl AlgorithmBuilder<NoMessageBus, NoAlgorithmParameters> {
    pub fn new() -> Self {
        AlgorithmBuilder {
            message_bus: NoMessageBus,
            algorithm_parameters: NoAlgorithmParameters,
        }
    }
}

impl<'a> AlgorithmBuilder<WithMessageBus<AlgorithmEvent>, WithAlgorithmParameters<'a>> {
    pub fn build(self) -> Algorithm<'a> {
        Algorithm {
            message_bus: self.message_bus.0,
            algorithm_parameters: self.algorithm_parameters.0,
        }
    }
}

impl<P> AlgorithmBuilder<NoMessageBus, P> {
    pub fn with_message_bus<M>(
        self,
        message_bus: MessageBus<M>,
    ) -> AlgorithmBuilder<WithMessageBus<M>, P> {
        AlgorithmBuilder {
            message_bus: WithMessageBus(message_bus),
            algorithm_parameters: self.algorithm_parameters,
        }
    }
}

impl<B> AlgorithmBuilder<B, NoAlgorithmParameters> {
    pub fn with_algorithm_parameters(
        self,
        algorithm_parameters: AlgorithmParameters,
    ) -> AlgorithmBuilder<B, WithAlgorithmParameters> {
        AlgorithmBuilder {
            message_bus: self.message_bus,
            algorithm_parameters: WithAlgorithmParameters(algorithm_parameters),
        }
    }
}
