use crate::algorithm::evaluator::Evaluate;
use std::ops::{Add, Deref, Mul, Sub};

pub struct Population<'a> {
    individuals: Vec<Individual>,
    evaluator: &'a dyn Evaluate,
}

impl Deref for Population<'_> {
    type Target = Vec<Individual>;

    fn deref(&self) -> &Self::Target {
        &self.individuals
    }
}

impl<'a> Population<'a> {
    pub fn new(individuals: Vec<Individual>, evaluator: &'a dyn Evaluate) -> Self {
        Self {
            individuals,
            evaluator,
        }
    }

    pub fn best(&self) -> &Individual {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Individual(pub Vec<f64>);

impl Individual {
    pub fn fitness(&self, evaluate: &dyn Evaluate) -> f64 {
        evaluate.evaluate(self)
    }
}

impl Sub for Individual {
    type Output = Individual;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Mul<f64> for Individual {
    type Output = Individual;

    fn mul(self, rhs: f64) -> Self::Output {
        todo!()
    }
}

impl Add for Individual {
    type Output = Individual;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}
