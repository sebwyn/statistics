use std::{collections::{HashMap, BTreeMap}, hash::Hash, ops::Range};

use super::Model;

#[derive(Debug, Clone)]
pub struct Pmf<T> {
    pub freqs: HashMap<T, f64>, //boxing technique used when plotting
}

//this datastructure is tricky, the key here is the start of a range
//uses step to bucket
#[derive(Clone)]
pub struct BucketedPmf {
    pub buckets: Vec<(f64, f64)>,
    pub step: (f64, f64)
}

pub struct Bucket {
    s: f64,
    e: f64
}

impl Bucket {
    fn contains(&self, val: f64) -> bool {
        self.s < val && val < self.e
    }
}

pub trait AsBucketed : Model {
    fn bucket(&self, step: (f64, f64)) -> BucketedPmf;
}

impl<T> AsBucketed for Pmf<T> 
where
    T: Into<f64> + Copy
{
    fn bucket(&self, step: (f64, f64)) -> BucketedPmf
    where
        Pmf<T>: Model
    {
        let range = self.range();
        let bucket_count: i128 = (((range.end - range.start) / step.0) as i128) + 1;
        println!("{} {} {} {}", range.start, range.end, step.0, (range.end - range.start) / step.0);
        let mut range_buckets: Vec<(Bucket, f64)> = (0..bucket_count)
            .map(|i| -> (Bucket, f64) {
                let start = range.start + (i as f64) * step.0;
                let end = range.start + ((i + 1) as f64) * step.0;
                (Bucket { s: start, e: end }, 0f64)
            })
            .collect();
        //create buckets
        let freq_iter = self.freqs.iter()
            .map(|(x, p)| ->  (f64, f64) {((*x).into(), *p)});
        for (x, p) in  freq_iter {
            for (bx, px) in range_buckets.iter_mut() {
                if bx.contains(x) {
                    *px += p;
                    break
                }
            }
        }
        let buckets: Vec<(f64, f64)> = range_buckets.into_iter().map(|(r, p)| (r.s, p)).collect();

        BucketedPmf { buckets, step }
    }
}

impl<T> Pmf<T> 
where 
    T: Into<f64>
{
    //with default step paramaters
    fn new() -> Self {
        Self {
            freqs: HashMap::new()
        }
    }

    pub fn normalize(&mut self) {
        let total: f64 = self.freqs.values().sum();  
        self.freqs.values_mut().for_each(|p| {
            *p /= total;
        }) 
    }

}

impl<T> From<Vec<T>> for Pmf<T>
where
    T: Hash + Eq + Into<f64> + Copy,
{
    fn from(vec: Vec<T>) -> Self {
        let mut pmf: Pmf<T> = Pmf::new();

        //count values in vec
        vec.iter().for_each(|x| {
            if let Some(v) = pmf.freqs.get_mut(x) {
                *v += 1f64;
            } else {
                pmf.freqs.insert(*x, 1f64);
            }
        });

        pmf.normalize();
        pmf
    }
}

impl<T> Model for Pmf<T>
where
    T: Into<f64> + Copy,
{
    fn mean(&self) -> f64 {
        self.freqs
            .iter()
            .map(|(x, p)| p * Into::<f64>::into(*x))
            .sum()
    }

    fn variance(&self) -> f64 {
        let mean = self.mean();
        self.freqs
            .iter()
            .map(|(x, p)| p * ((Into::<f64>::into(*x) - mean).powf(2.0)))
            .sum()
    }

    fn range(&self) -> Range<f64> {
        let mut lowest_x = f64::MAX;
        let mut highest_x = f64::MIN;
        for x in self.freqs.keys().into_iter().map(|k| Into::<f64>::into(*k)) {
            lowest_x = f64::min(lowest_x, x);
            highest_x = f64::max( highest_x, x);
        }
        
        Range { start: lowest_x, end: highest_x }
    }
}
