use plotters::prelude::*;
use std::{collections::BTreeMap, hash::Hash};

use crate::stats::cdf::*;

use super::Plot;
use super::plot_many::*;

impl<T> Plot for Cdf<T>
where
    T: Ord + Hash + Eq + Copy + Into<i128>,
{
    //temporarily use i128 as the base index type
    fn plot(&self, file: &str, name: &str){
        let x_min = self.min_x();
        let x_max = self.max_x();

        let paths = self.get_lines();

        //specify chart
        let size = (640, 480);
        let x_range = (x_min.into()..x_max.into()).step(1i128);
        let y_range = (0f32..1f32).step(0.01);

        let drawing_area = BitMapBackend::new(file, size).into_drawing_area();
        drawing_area.fill(&WHITE).unwrap(); //set a background color

        //construct our chart per specifications
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption(name, ("sans-serif", 50).into_font())
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_range, y_range)
            .unwrap();

        chart.configure_mesh().disable_mesh().draw().unwrap();
        chart.draw_series(paths.into_iter()).unwrap();
    }
}

impl<T> PlotMany<i128, f32> for Cdf<T> 
where
    T: Copy + Hash + Ord + Eq +Into<i128>
{
    fn get_chart_data(&self) -> PlotData<i128, f32> {
        PlotData::<i128, f32>::Lines(self.get_lines())
    }

    fn get_min(&self) -> (i128, f32) {
        (Into::<i128>::into(self.min_x()), 0f32)
    }

    fn get_max(&self) -> (i128, f32) {
        (Into::<i128>::into(self.max_x()), 1f32)
    }
}

impl<T> Cdf<T>
where
    T: Copy + Hash + Ord + Eq + Into<i128>,
{
    fn get_lines(&self) -> Vec<PathElement<(i128, f32)>> {
        let x_min = self.min_x();
        let x_max = self.max_x();

        let cdf: BTreeMap<&T, f32> = self.get_percentiles();

        let mut lines = Vec::new();

        let cdf_v: Vec<(i128, f32)> = [(x_min.into(), 0f32)]
            .into_iter()
            .chain(
                cdf.into_iter()
                    .map(|(x, y)| (x.clone().into(), y))
                    .chain([(x_max.into(), 1f32)].into_iter()),
            )
            .collect();
        for s in cdf_v.windows(3) {
            let (prev, curr, next) = (s[0], s[1], s[2]);
            let points = vec![(curr.0, prev.1), (curr.0, curr.1), (next.0, curr.1)];
            lines.push(PathElement::new(points, BLUE.filled()));
        }

        lines
    }
}
