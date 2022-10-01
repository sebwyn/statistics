use plotters::{prelude::*, style::WHITE};

pub struct Point<Coord> {
    circle: Circle<Coord, i32>
}

impl<Coord> Point<Coord> {
    fn new<S>(coord: Coord, style: S) -> Self 
    where
        S: plotters::style::Color 
    {
        Self {
            circle: Circle::new(coord, 2, style)
        }
    }
}

pub enum PlotData<X, Y> {
    Points(Vec<Point<(X, Y)>>),
    Lines(Vec<PathElement<(X, Y)>>),
    Rectangles(Rectangle<(X, Y)>)
}

//returns data with different typed dimensions
pub trait PlotMany<X, Y> {

    fn get_chart_data(&self) -> PlotData<X, Y>;

    fn get_min(&self) -> (X, Y);
    fn get_max(&self) -> (X, Y) ; 
}

fn plot_many<X, Y>(file: &str, size: (u32, u32), series: Vec<Box<dyn PlotMany<X, Y>>>) 
where
    X::max
{
    //get the largest min/max pairs and use at chart ranges

    //this setup won't work when X, or Y is an f32 yikes!
    let min_x = X
    let max_y
    for o in series.iter() {

    }


    let max_x: X = series.iter().map(|o| o.get_max().0).max().unwrap();
    let max_y: Y = series.iter().map(|o| o.get_max().1).max().unwrap();    

    let drawing_area = BitMapBackend::new(file, size).into_drawing_area();
    drawing_area.fill(&WHITE).unwrap(); //set a background color

    let mut chart = ChartBuilder::on(&drawing_area)
            .caption(name, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_range, y_range)
            .unwrap();

    chart.configure_mesh().disable_mesh().draw().unwrap();

    let 

    chart.draw_series().unwrap();
}