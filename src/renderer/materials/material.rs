use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec3::Color;
use crate::renderer::scene::hittable::Hit;

pub struct Scatter {
  pub ray: Ray,
  pub attenuation: Color,
}

pub trait Material {
  fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
