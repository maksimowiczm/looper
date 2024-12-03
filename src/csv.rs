use crate::algorithm::evaluator::Evaluator;
use crate::algorithm::individual::Individual;
use crate::algorithm::AlgorithmParameters;
use crate::cli::Verbose;

pub fn print_csv_header(params: &AlgorithmParameters) {
    let size = params.domain.len();
    let variables = (0..size).fold(String::new(), |acc, i| acc + &format!("var_{},", i + 1));

    match params.verbose {
        Verbose::None => {}
        Verbose::Iteration => {
            println!(
                "iteration,individual_id,{},fitness",
                variables.trim_end_matches(',')
            );
        }
        Verbose::Finished => {
            println!(
                "iterations,time,{},fitness",
                variables.trim_end_matches(',')
            );
        }
    }
}

pub fn print_csv_iteration(iteration: usize, population: &[Individual], evaluator: &Evaluator) {
    population
        .iter()
        .enumerate()
        .for_each(|(ind_id, individual)| {
            let fitness = evaluator(individual);
            let variables_str = individual
                .iter()
                .fold(String::new(), |acc, v| acc + &format!("{},", v));

            println!(
                "{},{},{},{}",
                iteration,
                ind_id,
                variables_str.trim_end_matches(','),
                fitness
            );
        });
}

pub fn print_csv_finished(iterations: usize, time: f64, best: Individual, evaluator: &Evaluator) {
    let variables_str = best
        .iter()
        .fold(String::new(), |acc, v| acc + &format!("{},", v));

    let fitness = evaluator(&best);

    println!(
        "{},{},{},{}",
        iterations,
        time,
        variables_str.trim_end_matches(','),
        fitness
    );
}
