pub mod cdf;
pub mod pmf;
pub mod distribution;


pub trait Plot {
    type Coord; //coord may be moved to a template of plot, refactoring required!

    //todo implement logrange
    fn plot(&self, file: &str, name: &str, size: (u32, u32));

    fn get_min(&self) -> Self::Coord;
    fn get_max(&self) -> Self::Coord;

    fn set_step(&mut self, value: Self::Coord) -> Self;
    fn get_step(&self) -> Self::Coord;
}