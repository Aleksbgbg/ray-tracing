use crate::renderer::core::color::{COLOR_LIGHT_BLUE, COLOR_WHITE};
use crate::renderer::core::math::{self, Range};
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec2::Vec2;
use crate::renderer::core::vec3::{Color, Vec3};
use crate::renderer::core::{color, random};
use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::hittable::{Hit, Hittable};

pub struct RenderParams {
  pub last_pixel: Vec2<usize>,
  pub samples_per_pixel: usize,
  pub max_bounces: usize,
}

pub struct Scene {
  pub camera: Camera,
  pub world: Box<dyn Hittable>,
}

pub fn render_pixel(params: &RenderParams, scene: &Scene, pixel: Vec2<usize>) -> Color {
  let mut pixel_color = Color::default();

  for _ in 0..params.samples_per_pixel {
    let u = (pixel.x() as f64 + random::random()) / params.last_pixel.x() as f64;
    let v = (pixel.y() as f64 + random::random()) / params.last_pixel.y() as f64;

    let ray = scene.camera.get_ray((u, v));

    pixel_color += ray_color(&ray, scene.world.as_ref(), params.max_bounces);
  }

  color::calculate_color(pixel_color, params.samples_per_pixel)
}

fn random_point_in_unit_sphere() -> Vec3 {
  random::random().cbrt() * Vec3::random().unit()
}

fn random_point_in_normal_hemisphere(normal: &Vec3) -> Vec3 {
  let point = random_point_in_unit_sphere();

  if Vec3::dot(&point, normal) > 0.0 {
    point
  } else {
    -point
  }
}

fn random_point_on_unit_sphere() -> Vec3 {
  random_point_in_unit_sphere().unit()
}

#[allow(dead_code)]
enum DiffuseMethod {
  BounceInHemisphere,
  LambertianApproximate,
  TrueLambertian,
}

fn bounce_direction(normal: Vec3, method: DiffuseMethod) -> Vec3 {
  match method {
    DiffuseMethod::BounceInHemisphere => random_point_in_normal_hemisphere(&normal),
    DiffuseMethod::LambertianApproximate => normal + random_point_in_unit_sphere(),
    DiffuseMethod::TrueLambertian => normal + random_point_on_unit_sphere(),
  }
}

fn generate_bounce_ray(hit: &Hit) -> Ray {
  Ray::new(
    hit.point(),
    bounce_direction(hit.normal(), DiffuseMethod::TrueLambertian),
  )
}

fn ray_color(ray: &Ray, world: &dyn Hittable, bounce_depth: usize) -> Color {
  if bounce_depth == 0 {
    Color::default()
  } else if let Some(hit) = world.hit(ray, Range::new(0.001, f64::INFINITY)) {
    0.5 * ray_color(&generate_bounce_ray(&hit), world, bounce_depth - 1)
  } else {
    let direction = ray.direction().unit();
    let time = math::map_range(direction.y(), Range::new(-1.0, 1.0), Range::new(0.0, 1.0));

    color::linear_blend(COLOR_LIGHT_BLUE, COLOR_WHITE, time)
  }
}
