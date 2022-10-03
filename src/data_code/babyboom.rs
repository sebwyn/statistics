use std::{fs::File, io::{self, BufRead}};

use polars::prelude::*;

pub fn get_data() -> DataFrame {
    //parse data from the file
    
    let mut clock_times = Vec::new();
    let mut sexs = Vec::new();
    let mut weights = Vec::new();
    let mut min_times = Vec::new();

    let file = File::open("datasets/Babyboom.dat").unwrap();
    let lines = io::BufReader::new(file).lines();
    for l in lines.into_iter() {
        let line = l.unwrap();
        let mut line_iter = line.split_ascii_whitespace().map(|x| x.parse::<i64>().unwrap());

        clock_times.push(line_iter.next().unwrap());
        sexs.push(line_iter.next().unwrap());
        weights.push(line_iter.next().unwrap());
        min_times.push(line_iter.next().unwrap());
    }

    df! {
        "clock_time" => clock_times,
        "sex" => sexs,
        "weight" => weights,
        "min_time" => min_times,
    }.unwrap()
}