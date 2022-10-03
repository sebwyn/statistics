pub mod exponential;
pub mod gaussian;

pub trait Distribution {
    //fn sample(&self, count: u32) -> Vec<f64>;
    fn eval(&self, x: f64) -> f64;
}

pub trait Sample {
    fn sample(&self, x: u32) -> Vec<f64>;
}

pub trait AsCdf {
    type Dist;

    fn cdf(&self) -> DistributionCdf<Self::Dist>;
}

#[derive(Clone)]
pub struct DistributionCdf<T> {
    distribution: T
}