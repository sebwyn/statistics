use plotters::prelude::*;

use crate::stats::pmf::*;
use super::Plot;

impl Plot for BucketedPmf
{
    type Coord = (f64, f64);

    fn plot(&self, file: &str, name: &str, size: (u32, u32)) {
        let style = ShapeStyle {
            color: RGBAColor(0, 0, 255, 1f64),
            filled: true,
            stroke_width: 2,
        };

        let min = self.get_min();
        let max = self.get_max();
        let x_range = (min.0..max.0).step(self.step.0).use_round();
        let y_range = (0f64..max.1).step(self.step.1).use_round();

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
        
        let half_margin = 0.01f64;

        let rectangles = {
            let mut rectangles = Vec::new();
            for (x, y) in self.buckets.iter() {
                rectangles.push(Rectangle::new([(*x + half_margin, 0f64), (*x + self.step.0 - half_margin , *y)], style));
            }
            rectangles
        };
        chart.draw_series(rectangles.into_iter()).unwrap();
    }

    fn get_min(&self) -> Self::Coord {
        let mut lowest_coord = (f64::MAX, f64::MAX);
        for (x, y) in self.buckets.iter() {
            lowest_coord = (f64::min(lowest_coord.0, *x), f64::min(lowest_coord.1, *y));
        }
        lowest_coord
    }

    fn get_max(&self) -> Self::Coord {
        let mut highest_coord = (f64::MIN, f64::MIN);
        for (x, y) in self.buckets.iter() {
            highest_coord = (f64::max(highest_coord.0, *x), f64::max(highest_coord.1, *y));
        }
        highest_coord
    }

    fn set_step(&mut self, value: Self::Coord) -> Self {
        self.step = value;
        self.clone()
    }

    fn get_step(&self) -> Self::Coord {
        self.step.clone()
    }
}