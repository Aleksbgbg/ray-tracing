use crate::utils::random;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, RangeInclusive, Sub};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
  x: f64,
  y: f64,
  z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
  pub const fn same_components(component: f64) -> Vec3 {
    Vec3::new(component, component, component)
  }

  pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
    Vec3 { x, y, z }
  }

  pub fn random() -> Vec3 {
    Vec3::random_range(-1.0..=1.0)
  }

  pub fn random_range(range: RangeInclusive<f64>) -> Vec3 {
    Vec3::new(
      random::random_range(range.clone()),
      random::random_range(range.clone()),
      random::random_range(range.clone()),
    )
  }

  pub fn dot(left: &Vec3, right: &Vec3) -> f64 {
    (left.x() * right.x()) + (left.y() * right.y()) + (left.z() * right.z())
  }

  pub fn cross(left: &Vec3, right: &Vec3) -> Vec3 {
    Vec3::new(
      (left.y() * right.z()) - (left.z() * right.y()),
      (left.z() * right.x()) - (left.x() * right.z()),
      (left.x() * right.y()) - (left.y() * right.x()),
    )
  }

  pub const fn x(&self) -> f64 {
    self.x
  }

  pub const fn y(&self) -> f64 {
    self.y
  }

  pub const fn z(&self) -> f64 {
    self.z
  }

  pub fn tuple(&self) -> (f64, f64, f64) {
    (self.x(), self.y(), self.z())
  }

  pub fn length_squared(&self) -> f64 {
    (self.x() * self.x()) + (self.y() * self.y()) + (self.z() * self.z())
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn unit(&self) -> Vec3 {
    (*self) / self.length()
  }

  pub fn map<T: Fn(f64) -> f64>(&self, func: T) -> Vec3 {
    Vec3::new(func(self.x()), func(self.y()), func(self.z()))
  }
}

impl Neg for Vec3 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    self.map(|value| -value)
  }
}

impl AddAssign for Vec3 {
  fn add_assign(&mut self, rhs: Self) {
    self.x += rhs.x();
    self.y += rhs.y();
    self.z += rhs.z();
  }
}

impl MulAssign for Vec3 {
  fn mul_assign(&mut self, rhs: Self) {
    self.x *= rhs.x();
    self.y *= rhs.y();
    self.z *= rhs.z();
  }
}

impl DivAssign for Vec3 {
  fn div_assign(&mut self, rhs: Self) {
    self.x /= rhs.x();
    self.y /= rhs.y();
    self.z /= rhs.z();
  }
}

impl Add for Vec3 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
  }
}

impl Sub for Vec3 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    self + (-rhs)
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Self::Output {
    self.map(|value| value * rhs)
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    Vec3::new(self * rhs.x(), self * rhs.y(), self * rhs.z())
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f64) -> Self::Output {
    self.map(|value| value / rhs)
  }
}
