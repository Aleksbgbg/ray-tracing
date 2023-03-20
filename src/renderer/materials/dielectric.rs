use crate::renderer::core::color::COLOR_WHITE;
use crate::renderer::core::random;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec3::Vec3;
use crate::renderer::materials::material::{Material, Scatter};
use crate::renderer::scene::hittable::{Face, Hit};

fn reflectance(cos_theta: f64, refraction_ratio: f64) -> f64 {
  let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powi(2);
  r0 + ((1.0 - r0) * (1.0 - cos_theta).powi(5))
}

fn refract(ray_in: Vec3, normal_in: Vec3, cos_theta: f64, refraction_ratio: f64) -> Vec3 {
  let perpendicular = refraction_ratio * (ray_in + (normal_in * cos_theta));
  let parallel = normal_in * -((1.0 - perpendicular.length_squared()).abs().sqrt());

  perpendicular + parallel
}

pub struct Dielectric {
  refractive_index: f64,
}

impl Dielectric {
  pub fn new(refractive_index: f64) -> Self {
    Self { refractive_index }
  }
}

impl Material for Dielectric {
  fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
    let unit_direction = ray.direction().unit();
    let normal = hit.normal();
    let refraction_ratio = {
      let (refractive_index_in, refractive_index_out) = match hit.face() {
        Face::Front => (1.0, self.refractive_index),
        Face::Back => (self.refractive_index, 1.0),
      };

      refractive_index_in / refractive_index_out
    };
    let cos_theta = (-unit_direction).dot(&normal).min(1.0);
    let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

    let can_refract = (refraction_ratio * sin_theta) <= 1.0;
    let should_reflect = reflectance(cos_theta, refraction_ratio) > random::random(0.0..1.0);

    let direction = if can_refract && !should_reflect {
      refract(unit_direction, normal, cos_theta, refraction_ratio)
    } else {
      unit_direction.reflect(&normal)
    };

    Some(Scatter::new(Ray::new(hit.point(), direction), COLOR_WHITE))
  }
}
