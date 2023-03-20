use crate::renderer::core::vec3::Vec3;

pub fn random_point_in_unit_sphere() -> Vec3 {
  loop {
    let vector = Vec3::random(-1.0..=1.0);

    if vector.length_squared() < 1.0 {
      return vector;
    }
  }
}

fn random_point_in_normal_hemisphere(normal: &Vec3) -> Vec3 {
  let point = random_point_in_unit_sphere();

  if point.dot(normal) > 0.0 {
    point
  } else {
    -point
  }
}

fn random_point_on_unit_sphere() -> Vec3 {
  random_point_in_unit_sphere().unit()
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
      *(*normal + random_point_in_unit_sphere()).non_zero_or(normal)
    }
    DiffuseMethod::TrueLambertian => *(*normal + random_point_on_unit_sphere()).non_zero_or(normal),
  }
}
