use crate::algorithm::population::Individual;

pub trait Evaluate {
    fn evaluate(&self, individual: &Individual) -> f64;
}
