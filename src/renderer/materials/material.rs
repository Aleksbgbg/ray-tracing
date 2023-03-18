use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec3::Color;
use crate::renderer::scene::hittable::Hit;

pub struct Scatter {
  ray: Ray,
  attenuation: Color,
}

impl Scatter {
  pub fn new(ray: Ray, attenuation: Color) -> Self {
    Self { ray, attenuation }
  }

  pub fn ray(&self) -> &Ray {
    &self.ray
  }

  pub fn attenuation(&self) -> Color {
    self.attenuation
  }
}

pub trait Material {
  fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter>;
}
