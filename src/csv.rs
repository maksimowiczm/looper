use crate::algorithm::evaluator::Evaluator;
use crate::algorithm::individual::Individual;

pub fn print_csv_header(size: usize)  {
    // Print the csv header
    let variables = (0..size).fold(String::new(), |acc, i| acc + &format!("var_{},", i + 1));

    println!(
        "iteration,individual_id,{},fitness",
        variables.trim_end_matches(',')
    )
}

pub fn print_csv_iteration(iteration: usize, population: &[Individual], evaluator: &Evaluator) {
    population
        .iter()
        .enumerate()
        .for_each(|(ind_id, individual)| {
            let fitness = (evaluator)(individual);
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
