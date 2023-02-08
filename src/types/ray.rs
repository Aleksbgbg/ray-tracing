use super::vec3::{Point3, Vec3};

pub struct Ray {
  origin: Point3,
  direction: Vec3,
}

impl Ray {
  pub fn default() -> Ray {
    Ray::new(Point3::default(), Vec3::default())
  }

  pub fn new(origin: Point3, direction: Vec3) -> Ray {
    Ray { origin, direction }
  }

  pub fn origin(&self) -> Point3 {
    self.origin
  }

  pub fn direction(&self) -> Vec3 {
    self.direction
  }

  pub fn at(&self, time: f64) -> Point3 {
    self.origin + (time * self.direction)
  }
}
