use rand::Rng;
use std::ops::Range;

pub fn random() -> f64 {
  random_range(-1.0..1.0)
}

pub fn random_range(range: Range<f64>) -> f64 {
  rand::thread_rng().gen_range(range)
}
