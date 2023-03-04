use crate::renderer::core::diffuse::{self, DiffuseMethod};
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec3::Color;
use crate::renderer::materials::material::{Material, Scatter};
use crate::renderer::scene::hittable::Hit;

pub struct Lambertian {
  albedo: Color,
}

impl Lambertian {
  pub fn new(albedo: Color) -> Self {
    Self { albedo }
  }
}

impl Material for Lambertian {
  fn scatter(&self, _ray: &Ray, hit: &Hit) -> Option<Scatter> {
    Some(Scatter {
      ray: Ray::new(
        hit.point(),
        diffuse::bounce_direction(&hit.normal(), DiffuseMethod::TrueLambertian),
      ),
      attenuation: self.albedo,
    })
  }
}
