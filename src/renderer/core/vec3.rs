use crate::renderer::core::{math, random};
use rand::distributions::uniform::SampleRange;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
  x: f64,
  y: f64,
  z: f64,
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
  pub const fn new(x: f64, y: f64, z: f64) -> Self {
    Self { x, y, z }
  }

  pub const fn from(component: f64) -> Self {
    Vec3::new(component, component, component)
  }

  pub fn random<T>(range: T) -> Self
  where
    T: SampleRange<f64> + Clone,
  {
    Vec3::new(
      random::random(range.clone()),
      random::random(range.clone()),
      random::random(range),
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

  pub fn dot(&self, right: &Vec3) -> f64 {
    (self.x() * right.x()) + (self.y() * right.y()) + (self.z() * right.z())
  }

  pub fn cross(&self, right: &Vec3) -> Vec3 {
    Vec3::new(
      (self.y() * right.z()) - (self.z() * right.y()),
      (self.z() * right.x()) - (self.x() * right.z()),
      (self.x() * right.y()) - (self.y() * right.x()),
    )
  }

  pub fn length_squared(&self) -> f64 {
    self.dot(self)
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn unit(&self) -> Vec3 {
    debug_assert_ne!(
      self.length(),
      0.0,
      "Converting the zero vector to a unit vector is not defined."
    );

    (*self) / self.length()
  }

  pub fn map(&self, func: impl Fn(f64) -> f64) -> Vec3 {
    Vec3::new(func(self.x()), func(self.y()), func(self.z()))
  }

  pub fn all(&self, func: impl Fn(f64) -> bool) -> bool {
    func(self.x()) && func(self.y()) && func(self.z())
  }

  pub fn near_zero(&self) -> bool {
    self.all(math::near_zero)
  }

  pub fn non_zero_or<'a>(&'a self, val: &'a Vec3) -> &'a Vec3 {
    if self.near_zero() {
      val
    } else {
      self
    }
  }

  pub fn reflect(&self, normal: &Vec3) -> Vec3 {
    *self - (2.0 * self.dot(normal) * *normal)
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
    *self = *self + rhs;
  }
}

impl SubAssign for Vec3 {
  fn sub_assign(&mut self, rhs: Self) {
    *self = *self - rhs;
  }
}

impl MulAssign for Vec3 {
  fn mul_assign(&mut self, rhs: Self) {
    *self = *self * rhs;
  }
}

impl DivAssign for Vec3 {
  fn div_assign(&mut self, rhs: Self) {
    *self = *self / rhs;
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

impl Mul for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
  }
}

impl Mul<f64> for Vec3 {
  type Output = Vec3;

  fn mul(self, rhs: f64) -> Self::Output {
    self * Vec3::from(rhs)
  }
}

impl Mul<Vec3> for f64 {
  type Output = Vec3;

  fn mul(self, rhs: Vec3) -> Self::Output {
    rhs * self
  }
}

impl Div for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: Vec3) -> Self::Output {
    Vec3::new(self.x() / rhs.x(), self.y() / rhs.y(), self.z() / rhs.y())
  }
}

impl Div<f64> for Vec3 {
  type Output = Vec3;

  fn div(self, rhs: f64) -> Self::Output {
    self / Vec3::from(rhs)
  }
}

impl Div<Vec3> for f64 {
  type Output = Vec3;

  fn div(self, rhs: Vec3) -> Self::Output {
    rhs / self
  }
}
