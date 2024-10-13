use crate::algorithm::evaluator::Evaluator;
use std::ops::{Add, Deref, Mul, Sub};

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

impl Population {
    pub fn new(individuals: Vec<Individual>, evaluator: Evaluator) -> Self {
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
pub struct Individual(Vec<f64>);

impl Deref for Individual {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.0
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
