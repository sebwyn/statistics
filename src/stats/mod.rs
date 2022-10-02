use std::ops::Range;

pub mod pmf;
pub mod cdf;

pub trait Model {
    fn mean(&self) -> f64;
    fn variance(&self) -> f64;
    fn range(&self) -> Range<f64>;
}