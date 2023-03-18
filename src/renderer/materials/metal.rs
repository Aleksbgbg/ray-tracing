use crate::renderer::core::diffuse::random_point_in_unit_sphere;
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec3::{Color, Vec3};
use crate::renderer::materials::material::{Material, Scatter};
use crate::renderer::scene::hittable::Hit;

pub struct Metal {
  albedo: Color,
  fuzziness: f64,
}

impl Metal {
  pub fn new(albedo: Color, fuzziness: f64) -> Self {
    Self {
      albedo,
      fuzziness: fuzziness.clamp(0.0, 1.0),
    }
  }
}

impl Material for Metal {
  fn scatter(&self, ray: &Ray, hit: &Hit) -> Option<Scatter> {
    let reflected_ray = ray.direction().unit().reflect(&hit.normal());
    let scattered_ray = Ray::new(
      hit.point(),
      reflected_ray + (self.fuzziness * random_point_in_unit_sphere(&Vec3::default())),
    );

    if Vec3::dot(&scattered_ray.direction(), &hit.normal()) > 0.0 {
      Some(Scatter::new(scattered_ray, self.albedo))
    } else {
      None
    }
  }
}
