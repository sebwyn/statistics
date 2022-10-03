use statistics::data_code::babyboom;

use statistics::stats::analytic::*;
use statistics::stats::analytic::gaussian::*;
use statistics::graphing::distribution::*;
use statistics::stats::cdf::*;
use statistics::graphing::Plot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = babyboom::get_data();
    let min_time: Vec<i64> = df.column("min_time")?.i64()?.into_iter().map(|x| x.unwrap()).collect();

    let mut last: i64 = 0;
    let mut interarrival_times = Vec::new();
    for time in min_time.into_iter() {
        interarrival_times.push(time - last);
        last = time;
    }
    let interarrival_cdf = Cdf::from(interarrival_times);
    interarrival_cdf.plot("plots/babyboom.png", "Baby Interarrival Time", (640, 480));

    //test some distributions
    let gaussian = Gaussian::new(5f64, 1f64);
    gaussian.cdf().build_plottable((0.1, 0.001), 0f64, 10f64).plot("plots/gaussian.png", "Gaussian", (640, 480));

    Ok(())
}
