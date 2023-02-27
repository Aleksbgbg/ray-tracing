use super::hittable::{Hit, Hittable};
use super::quadratic::Quadratic;
use super::ray::Ray;
use super::vec3::Vec3;
use crate::utils::math::{self, Mathematical, Range};

pub struct Sphere {
  center: Vec3,
  radius: f64,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64) -> Self {
    Sphere { center, radius }
  }

  pub fn center(&self) -> Vec3 {
    self.center
  }

  pub fn radius(&self) -> f64 {
    self.radius
  }
}

impl Hittable for Sphere {
  fn hit(&self, ray: &Ray, hittable_range: Range<f64>) -> Option<Hit> {
    let ray_to_sphere = ray.origin() - self.center();

    Quadratic::new(
      ray.direction().length_squared(),
      2.0 * Vec3::dot(&ray.direction(), &ray_to_sphere),
      ray_to_sphere.length_squared() - math::square(self.radius()),
    )
    .find_real_roots()
    .iter()
    .filter(|root| root.is_within(hittable_range))
    .next()
    .map(|&root| Hit::new(root, ray, |point| point - self.center()))
  }
}
