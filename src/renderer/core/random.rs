use rand::distributions::uniform::SampleRange;
use rand::Rng;

pub fn random(range: impl SampleRange<f64>) -> f64 {
  rand::thread_rng().gen_range(range)
}
