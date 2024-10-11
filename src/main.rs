use crate::algorithm::builder::AlgorithmBuilder;
use crate::algorithm::mutator;
use algorithm::AlgorithmParameters;
use clap::Parser;
use message_bus::MessageBus;

mod algorithm;
mod message_bus;

// CLI arguments
#[derive(Parser)]
struct Args {
    #[clap(long)]
    mutation: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let parameters = match parse_algorithm_parameters(&args) {
        Ok(parameters) => parameters,
        Err(_) => {
            // do kaboom
            todo!()
        }
    };

    let message_bus = MessageBus::new(100);

    message_bus.subscribe(Box::new(|event| {
        println!("Received event: {:?}", event);
    }));

    let algorithm = AlgorithmBuilder::new()
        .with_message_bus(message_bus)
        .with_algorithm_parameters(parameters)
        .build();

    algorithm.run().await;
}

#[derive(Debug)]
enum ParameterError {
    InvalidMutation(mutator::MutatorParserError),
}

/// Parse the algorithm parameters from the command line arguments.
fn parse_algorithm_parameters(args: &Args) -> Result<AlgorithmParameters, ParameterError> {
    let mutator =
        mutator::parse_mutator(&args.mutation).map_err(ParameterError::InvalidMutation)?;

    todo!()
}
