use crate::algorithm::builder::AlgorithmBuilder;
use crate::algorithm::AlgorithmParameters;

mod algorithm;
mod message_bus;

#[tokio::main]
async fn main() {
    let message_bus = message_bus::MessageBus::new(1);

    message_bus.subscribe(Box::new(|event| {
        println!("Received event: {:?}", event);
    }));

    let algorithm = AlgorithmBuilder::new()
        .with_message_bus(message_bus)
        .with_algorithm_parameters(AlgorithmParameters {})
        .build();

    algorithm.run().await;
}
