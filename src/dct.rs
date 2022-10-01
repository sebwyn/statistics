use regex::Regex;
use std::str::FromStr;
use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashMap;

use polars::prelude::*;

#[derive(Debug)]
pub enum DctError {
    RowNotConstructed,
    NoColumnNumber,
    TypeNotSupported
}

#[derive(Debug, PartialEq)]
struct DctRow {
    column_number: i32,
    kind: String,
    name: String,
    length: i32,
    description: String
}

pub struct Dct {
    rows: Vec<DctRow>
}

#[derive(Debug)]
enum S {
    Byte(Vec<Option<i32>>),
    Int(Vec<Option<i64>>),
    Double(Vec<Option<f64>>),
    String(Vec<Option<String>>),
}

//using polars for dataframes

impl FromStr for DctRow {
    type Err = DctError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let re: Regex = Regex::new(r"_column\(([^)]*)\)").unwrap();
        let caps = match re.captures(line).and_then(|cap| Some(cap)) {
            Some(x) => x,
            None => return Err(DctError::NoColumnNumber)
        };
        let column_number = i32::from_str(&caps[1]).expect("Could not parse column number");
        let mut columns = line.split_whitespace();

        columns.next(); //skip column number
        let kind = String::from(columns.next().expect("Could not parse kind"));
        let name = String::from(columns.next().expect("Could not parse name"));
        //get length in characters from pattern
        let pattern = String::from(columns.next().expect("Could not parse pattern"));
        let re = Regex::new(r"%(\d+)[a-z]").unwrap();
        let caps = match re.captures(&pattern).and_then(|cap| Some(cap)) {
            Some(x) => x,
            None => return Err(DctError::RowNotConstructed)
        };
        let length = i32::from_str(&caps[1]).expect("Could not parse length!");
        let description = String::from(columns.next().and_then(|s| Some(s.replace("\"", ""))).expect("Could not parse description"));

        //println!("row: {}, {}, {}, {}, {}", column_number, kind, name, length, description);

        Ok(DctRow {column_number, kind, name, length, description})
    }
}

impl Dct {
    pub fn from_file(file: &str) -> Result<Dct, DctError> {
        let mut dct = Dct {rows: Vec::new()};
        let file = File::open(file).unwrap();
        let lines = io::BufReader::new(file).lines();
        for l in lines {
            let line = l.unwrap();
            let dct_row: DctRow = match DctRow::from_str(&line) {
                Ok(x) => x,
                Err(DctError::NoColumnNumber) => continue,
                Err(x) => return Err(x)
            };

            dct.rows.push(dct_row);
        }

        Ok(dct)
    }

    pub fn read_file(self, file: &str) -> Result<DataFrame, DctError> {
        
        let file = File::open(file).unwrap();

        //construct a hash map of vectors with optional types
        let mut m = HashMap::new();
        self.rows.iter().for_each(|row| {
            let vec = match row.kind.as_str() {
                "byte" => S::Byte(Vec::new()),
                "int" => S::Int(Vec::new()),
                "float" | "double" => S::Double(Vec::new()),
                "str12" => S::String(Vec::new()),
                _ => panic!("Kind not supported")
            };
            m.insert(row.name.as_str(), vec);
        });

        let lines = io::BufReader::new(file).lines();
        for l in lines {
            let line = l.unwrap();
            //println!("{}", line);
            self.rows.iter().for_each(|row| {
                //get a substring for the matching field
                let f: String = line.clone().chars().skip((row.column_number - 1) as usize).take(row.length as usize).collect();
                let field: &str = f.trim();
                match m.get_mut(row.name.as_str()).unwrap() {
                    S::Byte(ref mut x) => x.push(i32::from_str(field).ok()),
                    S::Int(ref mut x) => x.push(i64::from_str(field).ok()),
                    S::Double(ref mut x) => x.push(f64::from_str(field).ok()),
                    S::String(ref mut x) => x.push(Some(String::from(field)))
                }
            });
        }

        //convert the hash map to a polars dataframe
        let mut series: Vec<Series> = Vec::new();
        for (key, value) in &m {
            series.push(match value {
                S::Byte(x) => Series::new(key, x),
                S::Int(x) => Series::new(key, x),
                S::Double(x) => Series::new(key, x),
                S::String(x) => Series::new(key, x),
            });
        }
        let df = DataFrame::new(series).unwrap();

        Ok(df)
    }
}

