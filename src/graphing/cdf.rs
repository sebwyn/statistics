use plotters::prelude::*;
use std::{collections::BTreeMap, hash::Hash};

use crate::stats::cdf::*;

use crate::graphing::graph::*;

impl<T> Plot for Cdf<T>
where
    T: Ord + Hash + Eq + Copy + Into<i128>,
{
    type Coord = (i128, f64);

    //temporarily use i128 as the base index type
    fn plot(&self, file: &str, name: &str, size: (u32, u32)){
        let min = self.get_min();
        let max = self.get_max();
        println!("{} {} {} {}", min.0, min.1, max.0, max.1);

        //specify chart
        let x_range = min.0..max.0;
        let y_range = (0f64..1f64).step(0.01);

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

        let lines = {
            let cdf: BTreeMap<T, f64> = self.get_percentiles();
            let cdf_v: Vec<Self::Coord> = [(min.0, 0f64)]
            .into_iter()
            .chain(
                cdf
                .into_iter()
                .map(|(x, y)| (x.into(), y))
            .chain([(max.0, 1f64)].into_iter()),
            )
            .collect();

            let mut lines = Vec::new();
            for s in cdf_v.windows(3) {
                let (prev, curr, next) = (s[0], s[1], s[2]);
                let points = vec![(curr.0, prev.1), (curr.0, curr.1), (next.0, curr.1)];
                lines.push(PathElement::new(points, BLUE.filled()));
            }

            lines
        };

        chart.draw_series(lines.into_iter()).unwrap();
    }

    fn get_min(&self) -> Self::Coord {
        let min_x = self.get_percentiles().keys().into_iter().next().expect("Invalid Cdf").clone();
        (Into::<i128>::into(min_x), 0f64)
    }

    fn get_max(&self) -> Self::Coord {
        let max_x = self.get_percentiles().keys().into_iter().next_back().expect("Invalid Cdf").clone();
        (Into::<i128>::into(max_x), 1f64)
    }

    fn set_step(&mut self, _value: Self::Coord) -> Self {
        //no implementation step stuck at 1%
        self.clone()
    }

    fn get_step(&self) -> Self::Coord {
        //no implementation step stuck at 1%
        (1, 0.10f64)
    }
}
