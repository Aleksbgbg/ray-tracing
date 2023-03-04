use crate::renderer::core::random;
use crate::renderer::core::vec3::Vec3;

pub fn random_point_in_unit_sphere(default: &Vec3) -> Vec3 {
  *(random::random().cbrt() * Vec3::random().non_zero_or(default).unit()).non_zero_or(default)
}

fn random_point_in_normal_hemisphere(normal: &Vec3) -> Vec3 {
  let point = random_point_in_unit_sphere(normal);

  if Vec3::dot(&point, normal) > 0.0 {
    point
  } else {
    -point
  }
}

fn random_point_on_unit_sphere(normal: &Vec3) -> Vec3 {
  random_point_in_unit_sphere(normal).unit()
}

#[allow(dead_code)]
pub enum DiffuseMethod {
  BounceInHemisphere,
  LambertianApproximate,
  TrueLambertian,
}

pub fn bounce_direction(normal: &Vec3, method: DiffuseMethod) -> Vec3 {
  match method {
    DiffuseMethod::BounceInHemisphere => random_point_in_normal_hemisphere(normal),
    DiffuseMethod::LambertianApproximate => {
      *(*normal + random_point_in_unit_sphere(normal)).non_zero_or(normal)
    }
    DiffuseMethod::TrueLambertian => {
      *(*normal + random_point_on_unit_sphere(normal)).non_zero_or(normal)
    }
  }
}
