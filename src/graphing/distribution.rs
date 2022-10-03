use plotters::prelude::*;

use crate::stats::analytic::Distribution;

use super::Plot;

#[derive(Clone)]
pub struct PlottableDistribution<T> {
    step: (f64, f64), //only x-step
    min: f64,
    max: f64,
    pub distribution: T
}

pub trait AsPlottableDistribution<T> {
    fn build_plottable(&self, step: (f64, f64), min: f64, max: f64) -> PlottableDistribution<T>;
}

impl<T> AsPlottableDistribution<T> for T 
where 
    T: Distribution + Clone
{
    fn build_plottable(&self, step: (f64, f64), min: f64, max: f64) -> PlottableDistribution<T> {
        PlottableDistribution { step, min, max, distribution: self.clone() }
    }
}

impl<T> Plot for PlottableDistribution<T>
where 
    T: Distribution,
    PlottableDistribution<T>: Clone
{
    type Coord = (f64, f64);

    fn plot(&self, file: &str, name: &str, size: (u32, u32)) {
        let style = ShapeStyle {
            color: RGBAColor(0, 0, 255, 1f64),
            filled: true,
            stroke_width: 1,
        };

        let x_range = (self.min..self.max).step(self.step.0).use_round();
        let y_range = (0f64..1f64).step(self.step.1).use_round();

        //calculate an acceptable width for the rectangles
        let drawing_area = BitMapBackend::new(file, size).into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption(name, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_range, y_range)
            .unwrap();
        chart.configure_mesh().disable_mesh().draw().unwrap();

        let points = {
            let mut points = Vec::new();
            let x_iter_count = ((self.max - self.min) / self.step.0) as i64 + 1;
            let x_iter = (0..x_iter_count).map(|i| self.min + self.step.0 * i as f64);
            for x in x_iter {
                let p = self.distribution.eval(x);
                if p < 1f64 {
                    points.push(Circle::new((x, p), 1, style));
                }
            }

            points
        };
        
        chart.draw_series(points.into_iter()).unwrap();
    }

    fn get_min(&self) -> Self::Coord {
        (self.min, 0f64)
    }

    fn get_max(&self) -> Self::Coord {
        (self.max, 1f64)
    }

    fn set_step(&mut self, value: Self::Coord) -> Self {
        self.step = value;
        self.clone()
    }

    fn get_step(&self) -> Self::Coord {
        self.step
    }
}