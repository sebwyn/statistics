use statistics::data_code::fem_preg;
use statistics::data_code::runners;

use statistics::plot::Plot;
use statistics::stats::cdf::Cdf;

use rand::prelude::*;

use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df = runners::get_data();
    let pace: Vec<i32> = df
        .column("Pace")?
        .i64()?
        .into_iter()
        .map(|x| (x.unwrap() as i32))
        .collect();
    let pace_cdf: Cdf<i32> = Cdf::from(pace);

    pace_cdf.plot("plots/Pace.png", "Pace Cdf");

    let df: DataFrame = fem_preg::get_data();
    let live_and_valid = df.clone().lazy()
        .filter(col("outcome").eq(lit(1)))
        .filter(not(col("prglngth").is_null()))
        .collect()
        .unwrap();


    let first_preglngth: Vec<i32> = live_and_valid.clone().lazy()
        .filter(col("birthord").eq(lit(1)))
        .collect()?
        .column("prglngth")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap())
        .collect();
    let first_prglngth_cdf = Cdf::from(first_preglngth);

    let other_preglngth: Vec<i32> = live_and_valid.clone().lazy()
        .filter(col("birthord").neq(lit(1)))
        .collect()?
        .column("prglngth")?
        .i32()?
        .into_iter()
        .map(|x| x.unwrap())
        .collect();
    let other_prglngth_cdf = Cdf::from(other_preglngth);

    let first_birthweight = df.clone().lazy()

    prglngth_cdf.plot("plots/Preglength.png", "Preglength Cdf");

    let mut rng = thread_rng();
    let random: Vec<i32> = (1..100).map(|_| rng.gen_range(1..100)).collect();
    let random_cdf = Cdf::from(random);

    random_cdf.plot("plots/random.png", "Random Cdf");

    Ok(())
}
