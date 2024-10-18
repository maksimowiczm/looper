use crate::algorithm::{Algorithm, AlgorithmEvent};
use crate::cli::{parse_arguments, Args};
use clap::Parser;
use data_logger::log_iteration;
use message_bus::MessageBus;

mod algorithm;
mod cli;
mod message_bus;
mod data_logger;

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
            log_iteration(i as i64, p);
        }
        AlgorithmEvent::Finished(p) => {
            println!(
                "Finished. Best: {:?}",
                p.best(),
            );

            log_iteration(-1, p);
        }
    }
}
