use std::collections::BTreeSet;
use std::collections::HashMap;

use std::hash::Hash;
use core::fmt::Debug;

pub struct Cdf<T> {
    sample: BTreeSet<T>,
    freqs: HashMap<T, usize>,
}

impl<T> Cdf<T>
where
    T: Ord + Hash + Eq + Copy,
{
    pub fn new() -> Cdf<T> {
        Self {
            sample: BTreeSet::new(),
            freqs: HashMap::new(),
        }
    }

    pub fn insert(&mut self, v: T) {
        if (self.sample.contains(&v)) {
            let s = self
                .freqs
                .get_mut(&v)
                .expect("Somehow in set and not in hash");
            *s += 1;
        } else {
            self.sample.insert(v);
            self.freqs.insert(v, 1);
        }
    }

    pub fn eval(&self, v: T) -> f32 {
        let mut position = 0;
        for s in self.sample.iter() {
            if v > *s {
                position += self.freqs.get(s).expect("Somehow in set and not in hash");
            } else {
                break;
            }
        }
        let length: usize = self.freqs.values().sum();

        position as f32 / length as f32
    }

    pub fn construct_eval_map(&self) -> HashMap<T, f32> {
        let mut m: HashMap<T, f32> = HashMap::new();
        for v in self.sample.iter() { //could iterate over sample or freqs, one is probably more performant
            m.insert(*v, self.eval(*v));
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
        for s in vec.iter() {
            cdf.insert(*s);
        }
        cdf
    }
}

impl<T> Debug for Cdf<T> 
where
    T: Debug + Ord + Hash + Eq + Copy
{
    fn fmt(&self, _: &mut core::fmt::Formatter) -> Result<(), std::fmt::Error> {
        //create an object that stores the evaluation of every value in the cdf
        println!("{:?}", self.construct_eval_map());
        Ok(())
    }
}
