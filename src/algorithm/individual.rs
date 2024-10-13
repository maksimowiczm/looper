use std::ops::{Add, Deref, Mul, Sub};

#[derive(Debug, Clone)]
pub struct Individual(Vec<f64>);

impl Individual {
    pub fn new(values: Vec<f64>) -> Self {
        Self(values)
    }
}

impl Deref for Individual {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl FromIterator<f64> for Individual {
    fn from_iter<T: IntoIterator<Item = f64>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.iter()
            .zip(other.iter())
            .all(|(a, b)| (a - b).abs() < 1e-6)
    }
}

impl Sub for Individual {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        self.iter().zip(other.iter()).map(|(a, b)| a - b).collect()
    }
}

impl Mul<f64> for Individual {
    type Output = Self;

    fn mul(self, other: f64) -> Self::Output {
        self.iter().map(|v| v * other).collect()
    }
}

impl Add for Individual {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        self.iter().zip(other.iter()).map(|(a, b)| a + b).collect()
    }
}
