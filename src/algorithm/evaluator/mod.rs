use consts::PI;
use std::f64::consts;

pub type Evaluator = fn(&[f64]) -> f64;

pub fn rastrigin(x: &[f64]) -> f64 {
    let a = 10.;
    let n = x.len() as f64;

    let sum: f64 = x.iter().map(|i| i.powi(2) - a * (2. * PI * i).cos()).sum();

    a * n + sum
}

pub fn griewank(x: &[f64]) -> f64 {
    let sum_part: f64 = x.iter().map(|&i| i.powi(2) / 4000.).sum();
    let product_part: f64 = x
        .iter()
        .enumerate()
        .map(|(i, &xi)| (xi / ((i + 1) as f64).sqrt()).cos())
        .product();

    1. + sum_part - product_part
}
