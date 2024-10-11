use crate::algorithm::evaluator::Evaluate;
use std::ops::Deref;

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
    pub fn best(&self) -> &Individual {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Individual(pub Vec<f64>);
