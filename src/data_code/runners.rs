use polars::prelude::*;
use polars::datatypes::AnyValue::*;

pub fn get_data() -> DataFrame {
    let reader = CsvReader::from_path("datasets/James_Joyce_Ramble_10kMasters.csv").unwrap();
    parse_data(reader.finish().unwrap())
}

fn parse_time( mut df: DataFrame, col: &str) -> DataFrame {
    let sec_time: Vec<i64> = df.column(col).unwrap().iter().map(|a| {
        //parse times here
        let time: String = String::from(match a {
            Utf8(s) => s,
            _ => panic!("Not a string!")
        });
        let  mut i = time.split(":").map(|s| s.parse::<i64>().unwrap());
        let hour = i.next().unwrap();
        let min = i.next().unwrap();
        let sec = i.next().unwrap();
        hour * 3600 + min * 60 + sec
    }).collect();
    let s = Series::new(col, sec_time);
    
    df = df.drop(col).unwrap();
    df.with_column(s).unwrap();

    df
}

fn parse_data(mut df: DataFrame) -> DataFrame {
    df = parse_time(df, "Nettime");
    df = parse_time(df, "Pace");
    df = parse_time(df, "Guntime");

    df
}