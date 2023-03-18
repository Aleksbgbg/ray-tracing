use crate::renderer::core::math::{Mathematical, Range};
use crate::renderer::core::quadratic::Quadratic;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec3::Vec3;
use crate::renderer::materials::material::Material;
use crate::renderer::scene::hittable::{Hit, Hittable};
use std::sync::Arc;

pub struct Sphere {
  center: Vec3,
  radius: f64,
  material: Arc<dyn Material>,
}

impl Sphere {
  pub fn new(center: Vec3, radius: f64, material: Arc<dyn Material>) -> Self {
    Sphere {
      center,
      radius,
      material,
    }
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
      ray_to_sphere.length_squared() - self.radius().powi(2),
    )
    .find_real_roots()
    .iter()
    .find(|root| root.is_within(hittable_range))
    .map(|&root| {
      Hit::new(root, ray, Arc::clone(&self.material), |point| {
        point - self.center()
      })
    })
  }
}
