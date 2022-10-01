pub mod cdf;
pub mod plot_many;

pub trait Plot {
    fn plot(&self, file: &str, name: &str);
}