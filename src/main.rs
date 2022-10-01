use statistics::data_code::runners;
use statistics::stats::cdf::Cdf;

use statistics::plot::cdf::*;
use statistics::plot::plot::Plot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = runners::get_data();
    let pace_v: Vec<i32> = df
        .column("Pace")?
        .i64()?
        .into_iter()
        .map(|x| (x.unwrap() as i32))
        .collect();
    let pace_cdf: Cdf<i32> = Cdf::from(pace_v);

    pace_cdf.plot("plots/Pace.png", "Pace Cdf");

    Ok(())
}
