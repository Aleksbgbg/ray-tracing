use std::cmp::Ordering;

pub struct Quadratic {
  a: f64,
  b: f64,
  c: f64,
}

#[derive(PartialEq)]
enum Roots {
  TwoComplex,
  OneReal,
  TwoReal,
}

enum DisriminantRoot {
  Negative,
  Positive,
}

impl Quadratic {
  pub fn new(a: f64, b: f64, c: f64) -> Self {
    Self { a, b, c }
  }

  fn discriminant(&self) -> f64 {
    self.b.powi(2) - (4.0 * self.a * self.c)
  }

  fn roots_type(&self) -> Roots {
    match self.discriminant().total_cmp(&0.0) {
      Ordering::Less => Roots::TwoComplex,
      Ordering::Equal => Roots::OneReal,
      Ordering::Greater => Roots::TwoReal,
    }
  }

  fn find_root(&self, which: DisriminantRoot) -> f64 {
    let multiplier = match which {
      DisriminantRoot::Negative => -1.0,
      DisriminantRoot::Positive => 1.0,
    };

    (-self.b + (multiplier * self.discriminant().sqrt())) / (2.0 * self.a)
  }

  pub fn find_real_roots(&self) -> Vec<f64> {
    match self.roots_type() {
      Roots::TwoComplex => vec![],
      Roots::OneReal => vec![self.find_root(DisriminantRoot::Positive)],
      Roots::TwoReal => vec![
        self.find_root(DisriminantRoot::Negative),
        self.find_root(DisriminantRoot::Positive),
      ],
    }
  }
}
