use crate::algorithm::evaluator::{
    ackley, beale, booth, griewank, levy, michalewicz, rastrigin, rosenbrock, schwefel, sphere,
    Evaluator,
};
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
    #[clap(long, default_value = "none")]
    pub verbose: Verbose,
    #[clap(long, default_value = "min")]
    pub goal: Goal,
    #[clap(long, num_args = 2.., required = true, allow_hyphen_values = true)]
    pub variables: Vec<f64>,
}

#[derive(ValueEnum, Copy, Clone)]
pub enum Goal {
    Min,
    Max,
}

#[derive(ValueEnum, Copy, Clone)]
pub enum Verbose {
    None,
    Iteration,
    Finished,
}

#[derive(ValueEnum, Copy, Clone)]
pub enum Function {
    Rastrigin,
    Griewank,
    Sphere,
    Rosenbrock,
    Ackley,
    Schwefel,
    Levy,
    Beale,
    Michalewicz,
    Booth,
}

impl From<Function> for Evaluator {
    fn from(value: Function) -> Self {
        match value {
            Function::Rastrigin => rastrigin,
            Function::Griewank => griewank,
            Function::Sphere => sphere,
            Function::Rosenbrock => rosenbrock,
            Function::Ackley => ackley,
            Function::Schwefel => schwefel,
            Function::Levy => levy,
            Function::Beale => beale,
            Function::Michalewicz => michalewicz,
            Function::Booth => booth,
        }
    }
}

#[derive(Debug)]
pub enum ParseError {
    InvalidMutation,
    PopulationTooSmall(String),
    DomainMissingMaximum(usize),
    DomainInvalid(usize, f64, f64),
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::PopulationTooSmall(s) => write!(f, "Population size too small: {}", s),
            ParseError::InvalidMutation => write!(f, "Invalid mutation"),
            ParseError::DomainMissingMaximum(i) => {
                write!(f, "Domain missing max value for variable {}", i)
            }
            ParseError::DomainInvalid(i, from, to) => {
                write!(f, "Invalid domain for variable {}: {} > {}", i, from, to)
            }
        }
    }
}

pub fn parse_arguments(args: &Args) -> Result<AlgorithmParameters, ParseError> {
    let mutator = parse_mutator(&args.mutation).map_err(|_| ParseError::InvalidMutation)?;

    if args.population_size < mutator.required_population_size() {
        return Err(ParseError::PopulationTooSmall(
            "Not enough individuals for given mutation".into(),
        ));
    }

    let variables = args
        .variables
        .chunks(2)
        .enumerate()
        .map(|(i, v)| {
            if v.len() != 2 {
                return Err(ParseError::DomainMissingMaximum(i + 1));
            }

            if v[0] >= v[1] {
                return Err(ParseError::DomainInvalid(i + 1, v[0], v[1]));
            }

            Ok((v[0], v[1]))
        })
        .collect::<Result<_, _>>()?;

    let params = AlgorithmParameters {
        evaluator: args.function.into(),
        mutator,
        mutation_factor: args.mutation_factor,
        crossover_probability: args.crossover_probability,
        iterations: args.iterations,
        population_size: args.population_size,
        domain: variables,
        verbose: args.verbose,
        goal: args.goal,
    };

    Ok(params)
}
