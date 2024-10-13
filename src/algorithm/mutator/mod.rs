use crate::algorithm::individual::Individual;
use crate::algorithm::population::Population;
use rand::seq::SliceRandom;
use rand::thread_rng;

mod best_mutator;
mod current_mutator;
mod parser;
mod random_mutator;

pub use parser::parse_mutator;

pub trait Mutate {
    fn mutate(&self, factor: f64, current: &Individual, population: &Population) -> Individual;
    fn vector_size(&self) -> usize;
}

fn difference(
    start: &Individual,
    how_many: usize,
    factor: f64,
    mut population: Vec<&Individual>,
) -> Individual {
    population.shuffle(&mut thread_rng());

    population
        .chunks(2)
        .take(how_many)
        .fold(start.clone(), |current, chunk| {
            assert_eq!(
                chunk.len(),
                2,
                "You can't have more differences than half the population"
            );

            let (lhs, rhs) = (chunk[0].clone(), chunk[1].clone());
            let difference = (lhs - rhs) * factor;
            current + difference
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_difference() {
        // V1 = X1 + F * (X3 − X2)
        let x1 = Individual::new(vec![1., 2.]);
        let x2 = Individual::new(vec![3., 4.]);
        let x3 = Individual::new(vec![5., 1.]);
        let factor = 0.5;
        // V1 = [1, 2] + 0.5 * ([5, 1] - [3, 4]) = [1, 2] + 0.5 * [2, -3] = [1, 2] + [1, -1.5] = [2, 0.5]
        // OR
        // V1 = [1, 2] + 0.5 * ([3, 4] - [5, 1]) = [1, 2] + 0.5 * [-2, 3] = [1, 2] + [-1, 1.5] = [0, 3.5]

        let start = &x1;
        let population = vec![&x2, &x3];

        let result = difference(start, 2, factor, population);

        assert!(
            result == Individual::new(vec![2., 0.5]) || result == Individual::new(vec![0., 3.5])
        );
    }

    #[test]
    #[should_panic]
    fn test_difference_panic() {
        let x1 = Individual::new(vec![1., 2.]);
        let x2 = Individual::new(vec![3., 4.]);
        let x3 = Individual::new(vec![5., 1.]);
        let factor = 0.5;

        let start = &x1;
        let population = vec![&x2];

        difference(start, 2, factor, population);
    }
}
