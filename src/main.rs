use crate::algorithm::evaluator::Evaluator;
use crate::algorithm::{Algorithm, AlgorithmEvent};
use crate::cli::{parse_arguments, Args};
use crate::csv::{print_csv_finished, print_csv_header, print_csv_iteration};
use clap::Parser;
use message_bus::MessageBus;
use std::ops::Deref;

mod algorithm;
mod cli;
mod csv;
mod message_bus;

fn main() {
    let args = Args::parse();
    let params = match parse_arguments(&args) {
        Ok(p) => p,
        Err(e) => return eprintln!("Error parsing arguments: {}", e),
    };

    print_csv_header(&params);

    let mut message_bus =
        MessageBus::with_subscriber(move |e| handle_algorithm_event(e, &params.evaluator));

    Algorithm::new(&message_bus, params).run();

    message_bus.close();
    let _ = message_bus.join();
}

fn handle_algorithm_event(event: AlgorithmEvent, evaluator: &Evaluator) {
    match event {
        AlgorithmEvent::Iteration(i, p) => {
            print_csv_iteration(i, p.deref(), evaluator);
        }
        AlgorithmEvent::Finished(i, t, p) => {
            print_csv_finished(i, t, p, evaluator);
        }
    }
}
