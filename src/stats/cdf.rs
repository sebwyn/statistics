use std::collections::BTreeMap;

use core::fmt::Debug;
use std::hash::Hash;

pub struct Cdf<T> {
    sample: BTreeMap<T, usize>,
}

impl<T> Cdf<T> 
where
    T: Copy
{
    pub fn min_x(&self) -> T {
        *self
            .sample
            .keys()
            .into_iter()
            .next()
            .expect("Can't call min on empty Cdf")
    }

    pub fn max_x(&self) -> T {
        *self
            .sample
            .keys()
            .into_iter()
            .next_back()
            .expect("Can't call max on empty Cdf")
    }
}

impl<T> Cdf<T>
where
    T: Ord + Hash + Eq + Copy,
{
    pub fn new() -> Cdf<T> {
        Self {
            sample: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, k: T) {
        if let Some(v) = self.sample.get_mut(&k) {
            *v += 1;
        } else {
            self.sample.insert(k, 1);
        }
    }

    pub fn get_percentile(&self, key: T) -> f32 {
        let mut position = 0;
        for (k, count) in self.sample.iter() {
            if key > *k {
                position += count;
            } else {
                break;
            }
        }
        let length: usize = self.sample.values().into_iter().sum();

        position as f32 / length as f32
    }

    //essentiall normalize the data with respect
    pub fn get_percentiles(&self) -> BTreeMap<&T, f32> {
        let mut m = BTreeMap::new();

        let length: usize = self.sample.values().into_iter().sum();
        let mut position = 0;
        for (key, count) in self.sample.iter() {
            m.insert(key, position as f32 / length as f32);
            position += count;
        }

        m
    }
}

impl<T> From<Vec<T>> for Cdf<T>
where
    T: Ord + Hash + Eq + Copy,
{
    fn from(vec: Vec<T>) -> Self {
        let mut cdf = Cdf::<T>::new();
        for key in vec.iter() {
            cdf.insert(*key);
        }
        cdf
    }
}

impl<T> Debug for Cdf<T>
where
    T: Debug + Ord + Hash + Eq + Copy,
{
    fn fmt(&self, _: &mut core::fmt::Formatter) -> Result<(), std::fmt::Error> {
        //create an object that stores the evaluation of every value in the cdf
        println!("{:?}", self.get_percentiles());
        Ok(())
    }
}
