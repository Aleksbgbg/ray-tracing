use crate::renderer::core::color::{
  COLOR_BLUE, COLOR_GREEN, COLOR_LIGHT_BLUE, COLOR_RED, COLOR_WHITE, COLOR_YELLOW,
};
use crate::renderer::core::math::{self, Range};
use crate::renderer::core::ray::Ray;
use crate::renderer::core::vec2::Vec2;
use crate::renderer::core::vec3::Color;
use crate::renderer::core::{color, random};
use crate::renderer::scene::camera::Camera;
use crate::renderer::scene::hittable::Hittable;

#[derive(Default)]
pub struct RenderParams {
  pub debug: bool,
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
    let u = (pixel.x() as f64 + random::random(0.0..1.0)) / params.last_pixel.x() as f64;
    let v = (pixel.y() as f64 + random::random(0.0..1.0)) / params.last_pixel.y() as f64;

    let ray = scene.camera.get_ray((u, v));

    pixel_color += ray_color(&ray, scene.world.as_ref(), params.max_bounces, params.debug);
  }

  color::calculate_color(pixel_color, params.samples_per_pixel)
}

fn ray_color(
  ray: &Ray,
  world: &dyn Hittable,
  bounce_depth: usize,
  debug_background: bool,
) -> Color {
  if bounce_depth == 0 {
    Color::default()
  } else if let Some(hit) = world.hit(ray, Range::new(0.001, f64::INFINITY)) {
    if let Some(scatter) = hit.material().scatter(ray, &hit) {
      scatter.attenuation() * ray_color(scatter.ray(), world, bounce_depth - 1, debug_background)
    } else {
      Color::default()
    }
  } else {
    let direction = ray.direction().unit();

    if debug_background {
      match (direction.x() >= 0.0, direction.y() >= 0.0) {
        (true, true) => COLOR_RED,
        (true, false) => COLOR_GREEN,
        (false, true) => COLOR_BLUE,
        (false, false) => COLOR_YELLOW,
      }
    } else {
      let time = math::map_range(direction.y(), Range::new(-1.0, 1.0), Range::new(0.0, 1.0));
      color::linear_blend(COLOR_LIGHT_BLUE, COLOR_WHITE, time)
    }
  }
}
