use polars::prelude::*;
use crate::dct::Dct;

pub fn get_data() -> DataFrame {
    let dct = Dct::from_file("datasets/2002FemPreg.dct").unwrap();
    let data = dct.read_file("datasets/2002FemPreg.dat").unwrap();
    clean_fem_preg(data)
}

fn clean_fem_preg(mut df: DataFrame) -> DataFrame {
    //convert agepreg to a value in years
    df = df
        .lazy()
        .with_column(col("agepreg") / lit(100f32))
        .collect()
        .unwrap();

    //clean 97, 98, 99 from birthweight_lb, and birthweight_oz which are error codes in the dataset and replace with null
    df = df
        .lazy()
        .with_column(
            when(
                col("birthwgt_lb")
                    .eq(lit(97))
                    .or(col("birthwgt_lb").eq(lit(98)))
                    .or(col("birthwgt_lb").eq(lit(99))),
            )
            .then(lit(Null {}))
            .otherwise(col("birthwgt_lb"))
            .keep_name(),
        )
        .collect()
        .unwrap();

    df = df
        .lazy()
        .with_column(
            when(
                col("birthwgt_oz")
                    .eq(lit(97))
                    .or(col("birthwgt_oz").eq(lit(98)))
                    .or(col("birthwgt_oz").eq(lit(99))),
            )
            .then(lit(Null {}))
            .otherwise(col("birthwgt_oz"))
            .keep_name(),
        )
        .collect()
        .unwrap();

    //compute a birthweight from oz and lbs
    df = df
        .lazy()
        .with_column(
            (col("birthwgt_lb") + lit(1.0 / 16.0) * col("birthwgt_oz")).alias("computed_birthwgt"),
        )
        .collect()
        .unwrap();

    df
}