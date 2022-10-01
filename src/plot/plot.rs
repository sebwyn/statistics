//structs that are plottable should implement this trait
pub trait Plot {
    fn plot(&self, file: &str, name: &str);
}
