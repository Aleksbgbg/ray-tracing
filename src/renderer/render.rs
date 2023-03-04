use crate::renderer::core::color::{COLOR_LIGHT_BLUE, COLOR_WHITE};
use crate::renderer::core::math::{self, Range};
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec2::Vec2;
use crate::renderer::core::vec3::{Color, Vec3};
use crate::renderer::core::{color, random};
use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::hittable::Hittable;

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

fn unit_sphere_random_point() -> Vec3 {
  loop {
    let point = Vec3::random();

    if point.length_squared() < 1.0 {
      break point;
    }
  }
}

fn ray_color(ray: &Ray, world: &dyn Hittable, bounce_depth: usize) -> Color {
  if bounce_depth == 0 {
    Color::default()
  } else if let Some(hit) = world.hit(ray, Range::new(0.001, f64::INFINITY)) {
    let bounce_ray = Ray::new(hit.point(), hit.normal() + unit_sphere_random_point());
    0.5 * ray_color(&bounce_ray, world, bounce_depth - 1)
  } else {
    let direction = ray.direction().unit();
    let time = math::map_range(direction.y(), Range::new(-1.0, 1.0), Range::new(0.0, 1.0));

    color::linear_blend(COLOR_LIGHT_BLUE, COLOR_WHITE, time)
  }
}
