use crate::algorithm::population::{Individual, Population};

pub trait Mutate {
    fn mutate(&self, factor: f64, current: &Individual, population: &Population) -> Individual;
}
