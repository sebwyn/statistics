pub mod pmf;
pub mod cdf;
pub mod analytic;
pub mod graphing;

pub trait Model {
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
}