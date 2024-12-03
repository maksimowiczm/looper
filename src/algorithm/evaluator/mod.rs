use consts::PI;
use std::f64::consts;

pub type Evaluator = fn(&[f64]) -> f64;

// https://www.sfu.ca/~ssurjano/rastr.html
// global minimum at f(0, 0, ..., 0) = 0 | bounds [-5.12, 5.12]
pub fn rastrigin(x: &[f64]) -> f64 {
    let a = 10.;
    let n = x.len() as f64;

    let sum: f64 = x.iter().map(|i| i.powi(2) - a * (2. * PI * i).cos()).sum();

    a * n + sum
}

// https://www.sfu.ca/~ssurjano/griewank.html
// global minimum at f(0, 0, ..., 0) = 0 | bounds [-600, 600]
pub fn griewank(x: &[f64]) -> f64 {
    let sum_part: f64 = x.iter().map(|&i| i.powi(2) / 4000.).sum();
    let product_part: f64 = x
        .iter()
        .enumerate()
        .map(|(i, &xi)| (xi / ((i + 1) as f64).sqrt()).cos())
        .product();

    1. + sum_part - product_part
}

//https://www.sfu.ca/~ssurjano/spheref.html
// global minimum at f(0, 0, ..., 0) = 0 | bounds [-5.12, 5.12]
pub fn sphere(x: &[f64]) -> f64 {
    x.iter().map(|&xi| xi.powi(2)).sum()
}

// https://www.sfu.ca/~ssurjano/rosen.html
// global minimum at f(1, 1, ..., 1) = 0 | bounds [-5, 10]
pub fn rosenbrock(x: &[f64]) -> f64 {
    x.windows(2)
        .map(|w| 100. * (w[1] - w[0].powi(2)).powi(2) + (w[0] - 1.).powi(2))
        .sum()
}

// https://www.sfu.ca/~ssurjano/ackley.html
// global minimum at f(0, 0, ..., 0) = 0 | bounds [-32, 32]
pub fn ackley(x: &[f64]) -> f64 {
    let n = x.len() as f64;
    let sum1: f64 = x.iter().map(|&xi| xi.powi(2)).sum();
    let sum2: f64 = x.iter().map(|&xi| (2. * PI * xi).cos()).sum();
    -20. * (-0.2 * (sum1 / n).sqrt()).exp() - (sum2 / n).exp() + 20. + consts::E
}

// https://www.sfu.ca/~ssurjano/schwef.html
// global minimum at f(420.9687, 420.9687, ..., 420.9687) = 0 | bounds [-500, 500]
pub fn schwefel(x: &[f64]) -> f64 {
    let sum: f64 = x.iter().map(|&xi| xi * (xi.abs()).sin()).sum();
    418.9829 * x.len() as f64 - sum
}

// https://www.sfu.ca/~ssurjano/levy.html
// global minimum at f(1, 1, ..., 1) = 0 | bounds [-10, 10]
pub fn levy(x: &[f64]) -> f64 {
    let w: Vec<f64> = x.iter().map(|xi| 1. + (xi - 1.) / 4.).collect();
    let sum: f64 = w
        .windows(2)
        .map(|w| (w[0] - 1.).powi(2) * (1. + 10. * (w[1] * 2. * PI).sin().powi(2)))
        .sum();
    sum + (w[w.len() - 1] - 1.).powi(2) * (1. + (2. * PI * w[w.len() - 1]).sin().powi(2))
}

// https://www.sfu.ca/~ssurjano/beale.html
// global minimum at f(3, 0.5) = 0 | bounds [-4.5, 4.5]
pub fn beale(x: &[f64]) -> f64 {
    x.windows(2)
        .map(|pair| {
            let (x0, x1) = (pair[0], pair[1]);
            (1.5 - x0 + x0 * x1).powi(2)
                + (2.25 - x0 + x0 * x1.powi(2)).powi(2)
                + (2.625 - x0 + x0 * x1.powi(3)).powi(2)
        })
        .sum()
}

// https://www.sfu.ca/~ssurjano/michal.html
// global minimum at f(2.20, 1.57) = -1.8013 | bounds [0, 3.14]
pub fn michalewicz(x: &[f64]) -> f64 {
    let m = 10;
    x.iter()
        .enumerate()
        .map(|(i, &xi)| (-1.0) * (xi * (i as f64 + 1.).sin()).powi(2 * m))
        .sum()
}

// https://www.sfu.ca/~ssurjano/booth.html
// global minimum at f(1, 3) = 0 | bounds [-10, 10]
pub fn booth(x: &[f64]) -> f64 {
    x.windows(2)
        .map(|pair| {
            let (x0, x1) = (pair[0], pair[1]);
            (x0 + 2. * x1 - 7.).powi(2) + (2. * x0 + x1 - 5.).powi(2)
        })
        .sum()
}
