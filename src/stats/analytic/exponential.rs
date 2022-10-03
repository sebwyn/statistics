use std::f64::consts::E;

use crate::stats::Model;

use super::Distribution;

#[derive(Clone)]
struct Exponential {
    lambda: f64
}

impl Model for Exponential {
    fn mean(&self) -> f64 {
        1f64 / self.lambda
    }

    fn variance(&self) -> f64 {
        1f64 / self.lambda.powf(2f64)
    }
}

impl Distribution for Exponential {
    fn eval(&self, x: f64) -> f64 {
        1f64 - E.powf(self.lambda * x)
    }
}