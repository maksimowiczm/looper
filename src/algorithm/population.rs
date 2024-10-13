use crate::algorithm::evaluator::Evaluator;
use crate::algorithm::individual::Individual;
use std::ops::Deref;

pub struct Population {
    individuals: Vec<Individual>,
    evaluator: Evaluator,
}

impl Deref for Population {
    type Target = Vec<Individual>;

    fn deref(&self) -> &Self::Target {
        &self.individuals
    }
}

#[derive(Debug)]
pub enum PopulationError {
    EmptyPopulation,
}

impl Population {
    pub fn new(
        individuals: Vec<Individual>,
        evaluator: Evaluator,
    ) -> Result<Self, PopulationError> {
        if individuals.is_empty() {
            return Err(PopulationError::EmptyPopulation);
        }

        Ok(Self {
            individuals,
            evaluator,
        })
    }

    pub fn best(&self) -> &Individual {
        // the lower, the better

        self.individuals
            .iter()
            .min_by(|lhs, rhs| {
                (self.evaluator)(lhs)
                    .partial_cmp(&(self.evaluator)(rhs))
                    .expect("Evaluator always returns a number")
            })
            .expect("Population is not empty")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sum(x: &[f64]) -> f64 {
        x.iter().sum()
    }

    #[test]
    fn test_best() {
        let individuals = vec![
            Individual::new(vec![5., 6.]),
            Individual::new(vec![1., 2.]),
            Individual::new(vec![3., 4.]),
        ];
        let population = Population::new(individuals, sum).unwrap();

        let best = population.best();

        assert_eq!(best, &Individual::new(vec![1., 2.]));
    }
}
