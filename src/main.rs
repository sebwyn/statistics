use statistics::stats::analytic::gaussian::*;
use statistics::stats::graphing::distribution::*;

use statistics::stats::graphing::Plot;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    /*let df = babyboom::get_data();
    let min_time: Vec<i64> = df.column("min_time")?.i64()?.into_iter().map(|x| x.unwrap()).collect();

    let mut last: i64 = 0;
    let mut interarrival_times = Vec::new();
    for time in min_time.into_iter() {
        interarrival_times.push(time - last);
        last = time;
    }
    let interarrival_cdf = Cdf::from(interarrival_times);
    interarrival_cdf.plot("plots/babyboom.png", "Baby Interarrival Time", (640, 480));
    */
    //test some distributions
    let gaussian = Gaussian::new(100f64, 5f64);
    gaussian.build_plottable((1f64, 0.001), 0f64, 200f64).plot("plots/gaussian.png", "Gaussian", (640, 480));

    Ok(())
}
