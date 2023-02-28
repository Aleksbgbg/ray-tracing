use rand::Rng;
use std::ops::RangeInclusive;

pub fn random() -> f64 {
  random_range(-1.0..=1.0)
}

pub fn random_range(range: RangeInclusive<f64>) -> f64 {
  rand::thread_rng().gen_range(range)
}
