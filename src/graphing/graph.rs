use plotters::prelude::*;

pub struct Point<Coord> {
    circle: Circle<Coord, i32>
}

impl<Coord> Point<Coord> {
    fn new(coord: Coord, size: i32, style: ShapeStyle) -> Self {
        Self {
            circle: Circle::new(coord, 5, style)
        }
    }
}

//probably want to refactor to allow for any drawable (trait: drawable and to<Points>)
pub enum PlotDataTypes<Coord> {
    Points(Vec<Point<Coord>>),
    Lines(Vec<PathElement<Coord>>),
    Rects(Vec<Rectangle<Coord>>)
}

pub trait Plot {
    type Coord; //coord may be moved to a template of plot, refactoring required!

    fn plot(&self, file: &str, name: &str);
    fn get_plot_data(&self) -> PlotDataTypes<Self::Coord>;

    fn get_min(&self) -> Self::Coord;
    fn get_max(&self) -> Self::Coord;
}
