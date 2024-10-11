use crate::algorithm::evaluator::Evaluate;
use crate::algorithm::mutator::{parse_mutator, MutatorParserError};
use crate::algorithm::AlgorithmParameters;
use clap::{Parser, ValueEnum};
use std::fmt::{Display, Formatter};

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    pub iterations: usize,
    #[clap(long)]
    pub population_size: usize,
    #[clap(long)]
    pub crossover_probability: f64,
    #[clap(long)]
    pub mutation: String,
    #[clap(long)]
    pub mutation_factor: f64,
    #[clap(long)]
    pub function: Function,
    // todo add variables
}

#[derive(ValueEnum, Clone)]
pub enum Function {
    Rastrigin,
    Griewank,
    Rosenbrock,
}

impl Function {
    pub fn new_with_vector_size(&self, vector_size: usize) -> Box<dyn Evaluate> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidMutation(MutatorParserError),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidMutation(e) => write!(f, "Invalid mutation: {}", e),
        }
    }
}

pub fn parse_arguments(args: &Args) -> Result<AlgorithmParameters, ParseError> {
    let mutator = parse_mutator(&args.mutation).map_err(ParseError::InvalidMutation)?;

    let evaluator = args.function.new_with_vector_size(mutator.vector_size());

    // todo validate variable counts, everything should be the same size

    let params = AlgorithmParameters {
        evaluator,
        mutator,
        mutation_factor: args.mutation_factor,
        crossover_probability: args.crossover_probability,
        iterations: args.iterations,
        population_size: args.population_size,
    };

    Ok(params)
}
