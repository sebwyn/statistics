use statistics::data_code::runners;

use statistics::graphing::graph::*;
use statistics::stats::cdf::Cdf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = runners::get_data();
    let pace: Vec<i32> = df
        .column("Pace")?
        .i64()?
        .into_iter()
        .map(|x| (x.unwrap() as i32))
        .collect();
    let pace_cdf: Cdf<i32> = Cdf::from(pace);

    pace_cdf.plot("plots/pace.png", "Pace CDF");

    Ok(())
}
