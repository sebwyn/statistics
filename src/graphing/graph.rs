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

    //continue to implement in a very uniform way so that i can refactor into a template
    fn plot(&self, file: &str, name: &str, size: (u32, u32));

    fn get_min(&self) -> Self::Coord;
    fn get_max(&self) -> Self::Coord;

    fn set_step(&mut self, value: Self::Coord) -> Self;
    fn get_step(&self) -> Self::Coord;
}

//maybe one day add a multiplot function, things are set up for it for sure
//for now it doesn't seem like too much overhead to leave things this way 

