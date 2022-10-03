use std::f64::consts::*;
use libm::erf;

use super::*;

const SQRT_PI: f64 = 1.77245385;

#[derive(Clone)]
pub struct Gaussian {
    mean: f64,
    std_deviation: f64
}

impl AsCdf for Gaussian {
    type Dist = Gaussian;

    fn cdf(&self) -> DistributionCdf<Gaussian> {
        //compute an integral here (integral is erf)
        DistributionCdf { distribution: self.clone() }
    }
}

impl Distribution for DistributionCdf<Gaussian> {
    fn eval(&self, x: f64) -> f64 {
        0.5f64 * (1f64 + erf((x - self.distribution.mean) / (SQRT_2 * self.distribution.std_deviation)))
    }
}

impl Gaussian {
    pub fn new(mean: f64, std_deviation: f64) -> Self {
        Self {
            mean,
            std_deviation
        }
    }
}

impl Distribution for Gaussian {
    fn eval(&self, x: f64) -> f64 {
        let exponent = ((x - self.mean) / self.std_deviation).powf(2f64);
        let coefficient =  1f64 / (self.std_deviation * SQRT_2 * SQRT_PI);
        let y = coefficient * (-0.5f64 * exponent).exp();
        //println!("{}", y);
        y
    }
}