use super::ray::Ray;
use super::vec3::{Point3, Vec3};
use crate::utils::math::Range;

pub enum Face {
  Front,
  Back,
}

pub struct Hit {
  time: f64,
  point: Point3,
  normal: Vec3,
  face: Face,
}

impl Hit {
  pub fn new(time: f64, ray: &Ray, normal_calc: impl FnOnce(Point3) -> Vec3) -> Self {
    let point = ray.at(time);
    let outward_normal = normal_calc(point).unit();
    let face = if Vec3::dot(&ray.direction(), &outward_normal) < 0.0 {
      Face::Front
    } else {
      Face::Back
    };

    Hit {
      time,
      point,
      normal: match face {
        Face::Front => outward_normal,
        Face::Back => -outward_normal,
      },
      face,
    }
  }

  pub fn time(&self) -> f64 {
    self.time
  }

  pub fn point(&self) -> Point3 {
    self.point
  }

  pub fn normal(&self) -> Vec3 {
    self.normal
  }
}

pub trait Hittable {
  fn hit(&self, ray: &Ray, hittable_range: Range<f64>) -> Option<Hit>;
}

type HittablePtr = Box<dyn Hittable>;

impl Hittable for Vec<HittablePtr> {
  fn hit(&self, ray: &Ray, hittable_range: Range<f64>) -> Option<Hit> {
    let mut vec = self
      .iter()
      .map(|value| value.hit(ray, hittable_range))
      .filter(|value| value.is_some())
      .flatten()
      .collect::<Vec<Hit>>();
    vec.sort_unstable_by(|a, b| b.time.total_cmp(&a.time));

    vec.pop()
  }
}
