use statistics::data_code::runners;

use statistics::stats::pmf::*;
use statistics::stats::cdf::*;
use statistics::graphing::graph::Plot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = runners::get_data();
    let pace: Vec<i32> = df
        .column("Pace")?
        .i64()?
        .into_iter()
        .map(|x| (x.unwrap() as i32))
        .collect();
    let pace_pmf: Pmf<i32> = Pmf::from(pace.clone());
    pace_pmf.bucket((20f64, 0.0001f64)).plot("plots/pace_pmf.png", "Pace Pmf", (640, 480));

    let pace_cdf: Cdf<i32> = Cdf::from(pace);
    pace_cdf.plot("plots/pace_cdf.png", "Pace cdf", (640, 480));

    Ok(())
}
