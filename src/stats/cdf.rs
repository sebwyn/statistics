use std::collections::BTreeMap;

use core::fmt::Debug;
use std::hash::Hash;

use super::pmf::*;

//this struct will always exist as a percentile map in the wild
#[derive(Clone)]
pub struct Cdf<T> {
    percentiles: BTreeMap<T, f64>,
    count: f64
}

impl<T> Cdf<T>
where
    T: Ord + Hash + Eq + Copy,
{
    fn new() -> Cdf<T> {
        Self {
            percentiles: BTreeMap::new(),
            count: 0f64
        }
    }

    //one of the things that I don't like about this 
    pub fn calc_percentile(&self, key: T) -> f64 {
        let mut percentile = 0f64;
        for (k, percent) in self.percentiles.iter() {
            if key > *k {
                percentile += *percent;
            } else {
                break;
            }
        }

        percentile
    }
    //
    fn to_percentiles(&self) -> Cdf<T> { 
        let mut m: BTreeMap<T, f64> = BTreeMap::new();

        let mut position = 0f64;
        for (key, count) in self.percentiles.iter() { //percentiles here is not normalized
            m.insert(*key, position / self.count);
            position += count;
        }

        Self {
            percentiles: m,
            count: self.count
        }
    }

    pub fn get_percentiles(&self) -> BTreeMap<T, f64> {
        self.percentiles.clone()
    }

}

impl<T> From<Pmf<T>> for Cdf<T> 
where
    T: Ord + Hash + Eq + Copy,
{
    fn from(pmf: Pmf<T>) -> Self {
        let mut cdf = Self::new();
        //both data structures are unique so not very much logic is needed
        for (k, v) in pmf.freqs.iter() {
            cdf.percentiles.insert(*k, *v);  
        }
        cdf.count = 1f64;

        cdf.to_percentiles()
    }
}

impl<T> From<Vec<T>> for Cdf<T>
where
    T: Ord + Hash + Eq + Copy,
{
    fn from(vec: Vec<T>) -> Self {
        let mut cdf = Cdf::<T>::new();
        for key in vec.iter() {
            //implement insert behavior
            if let Some(x) = cdf.percentiles.get_mut(key) {
                *x += 1f64;
            } else {
                cdf.percentiles.insert(*key, 1f64);
            }
        }
        cdf.count = vec.len() as f64;

        cdf.to_percentiles()
    }
}

impl<T> Debug for Cdf<T>
where
    T: Debug + Ord + Hash + Eq + Copy,
{
    fn fmt(&self, _: &mut core::fmt::Formatter) -> Result<(), std::fmt::Error> {
        //create an object that stores the evaluation of every value in the cdf
        println!("{:?}", self.percentiles);
        Ok(())
    }
}
