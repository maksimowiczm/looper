use crate::algorithm::evaluator::{griewank, rastrigin, Evaluator};
use crate::algorithm::mutator::parse_mutator;
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

#[derive(ValueEnum, Copy, Clone)]
pub enum Function {
    Rastrigin,
    Griewank,
    Rosenbrock,
}

impl From<Function> for Evaluator {
    fn from(value: Function) -> Self {
        match value {
            Function::Rastrigin => rastrigin,
            Function::Griewank => griewank,
            Function::Rosenbrock => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidMutation,
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidMutation => write!(f, "Invalid mutation"),
        }
    }
}

pub fn parse_arguments(args: &Args) -> Result<AlgorithmParameters, ParseError> {
    let mutator = parse_mutator(&args.mutation).map_err(|_| ParseError::InvalidMutation)?;

    // todo validate variables, everything should be the same size

    let params = AlgorithmParameters {
        evaluator: args.function.into(),
        mutator,
        mutation_factor: args.mutation_factor,
        crossover_probability: args.crossover_probability,
        iterations: args.iterations,
        population_size: args.population_size,
    };

    Ok(params)
}
