use std::fs::OpenOptions;
use crate::algorithm::population::Population;
use std::io::{Write};

static OUTPUT_FILE: &str = "result_log.csv";

pub fn log_iteration(iteration: i64, population: Population) {
    if iteration == 0 {
        let _ = std::fs::remove_file(OUTPUT_FILE);
    }

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(OUTPUT_FILE)
        .unwrap();

    if file.metadata().unwrap().len() == 0 {
        let num_variables = population.as_ref()[0].len();
        let mut header = String::from("iteration,individual_id");
        for i in 1..=num_variables {
            header.push_str(&format!(",var_{}", i));
        }
        header.push_str(",fitness");  // Add the Fitness column header at the end
        write!(file, "{}", header).expect("Unable to write data");
    }

    for (ind_id, individual) in population.as_ref().iter().enumerate() {
        let fitness = population.evaluate_for(individual);
        let variables_str = individual
            .iter()
            .map(|v| v.to_string())  // Convert each variable to string
            .collect::<Vec<String>>()
            .join(",");              // Join with commas

        write!(file, "\n{},{},{},{}", iteration, ind_id, variables_str, fitness.to_string())
            .expect("Unable to write data");
    }
}