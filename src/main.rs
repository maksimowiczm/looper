use crate::algorithm::{Algorithm, AlgorithmEvent};
use crate::cli::{parse_arguments, Args};
use clap::Parser;
use message_bus::MessageBus;
use std::ops::Deref;
use tokio::io::{stdout, AsyncWriteExt};

mod algorithm;
mod cli;
mod message_bus;

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let params = match parse_arguments(&args) {
        Ok(p) => p,
        Err(e) => return eprintln!("Error parsing arguments: {}", e),
    };

    let message_bus = MessageBus::with_subscriber(100, Box::new(handle_algorithm_event));

    Algorithm::new(message_bus, params).run().await;

    stdout()
        .flush()
        .await
        .expect("If it doesn't flush this program doesn't have any purpose");
}

fn handle_algorithm_event(event: AlgorithmEvent) {
    match event {
        AlgorithmEvent::Iteration(i, p) => {
            println!("{i}: {:?}", p.deref());
        }
        AlgorithmEvent::Finished(p) => {
            println!("Finished. Best: {:?}, Population {:?}", p.best(), p.deref());
        }
    }
}
