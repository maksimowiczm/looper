use crate::algorithm::{Algorithm, AlgorithmEvent};
use crate::cli::{parse_arguments, Args};
use clap::Parser;
use message_bus::MessageBus;

mod algorithm;
mod cli;
mod message_bus;

fn main() {
    let args = Args::parse();
    let params = match parse_arguments(&args) {
        Ok(p) => p,
        Err(e) => return eprintln!("Error parsing arguments: {}", e),
    };

    let mut message_bus = MessageBus::with_subscriber(handle_algorithm_event);

    Algorithm::new(&message_bus, params).run();

    message_bus.close();
    let _ = message_bus.join();
}

fn handle_algorithm_event(event: AlgorithmEvent) {
    match event {
        AlgorithmEvent::Iteration(i, p) => {
            println!("{i}: {:?}", p.as_ref());
        }
        AlgorithmEvent::Finished(p) => {
            println!(
                "Finished. Best: {:?}, Population {:?}",
                p.best(),
                p.as_ref()
            );
        }
    }
}
