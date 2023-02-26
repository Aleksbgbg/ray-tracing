use std::cmp::Ordering;

pub struct Quadratic {
  a: f64,
  b: f64,
  c: f64,
}

#[derive(PartialEq)]
pub enum Roots {
  OneReal,
  TwoReal,
  TwoComplex,
}

impl Quadratic {
  pub fn new(a: f64, b: f64, c: f64) -> Self {
    Self { a, b, c }
  }

  pub fn discriminant(&self) -> f64 {
    (self.b * self.b) - (4.0 * self.a * self.c)
  }

  pub fn roots(&self) -> Roots {
    match self.discriminant().total_cmp(&0.0) {
      Ordering::Less => Roots::TwoComplex,
      Ordering::Equal => Roots::OneReal,
      Ordering::Greater => Roots::TwoReal,
    }
  }
}
