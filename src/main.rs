use crate::algorithm::builder::AlgorithmBuilder;
use crate::algorithm::AlgorithmEvent;
use crate::cli::{parse_arguments, Args};
use clap::Parser;
use message_bus::MessageBus;

mod algorithm;
mod cli;
mod message_bus;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let params = match parse_arguments(&args) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error parsing arguments: {}", e);
            return;
        }
    };

    let message_bus = MessageBus::new(100);
    setup_ui(&message_bus);

    AlgorithmBuilder::new()
        .with_message_bus(message_bus)
        .with_algorithm_parameters(params)
        .build()
        .run()
        .await
}

fn setup_ui(message_bus: &MessageBus<AlgorithmEvent>) {
    message_bus.subscribe(Box::new(|event| {
        println!("Received event: {:?}", event);
    }));
}
