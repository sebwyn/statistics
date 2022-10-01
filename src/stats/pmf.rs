use std::{collections::HashMap, hash::Hash};

#[derive(Debug)]
pub struct Pmf<T> {
    freqs: HashMap<T, f64>,
}

impl<T> Pmf<T> {
    fn new() -> Self {
        Self {
            freqs: HashMap::new()
        }
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

impl<T> Pmf<T>
where
    T: Into<f64> + Copy,
{
    pub fn mean(&self) -> f64 {
        self.freqs
            .iter()
            .map(|(x, p)| p * Into::<f64>::into(*x))
            .sum()
    }

    pub fn variance(&self) -> f64 {
        let mean = self.mean();
        self.freqs
            .iter()
            .map(|(x, p)| p * ((Into::<f64>::into(*x) - mean).powf(2.0)))
            .sum()
    }

    pub fn normalize(&mut self) {
        let total: f64 = self.freqs.values().sum();  
        self.freqs.values_mut().for_each(|p| {
            *p /= total;
        }) 
    }
}
