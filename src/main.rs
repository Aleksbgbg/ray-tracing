use crate::types::camera::Camera;
use crate::types::hittable::Hittable;
use crate::types::ray::Ray;
use crate::types::sphere::Sphere;
use crate::types::vec3::{Color, Point3, Vec3};
use crate::utils::color::{COLOR_LIGHT_BLUE, COLOR_WHITE};
use crate::utils::math::Range;
use crate::utils::{color, math, random};

mod types;
mod utils;

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
  } else if let Some(hit) = world.hit(ray, Range::new(0.0, f64::INFINITY)) {
    let bounce_ray = Ray::new(hit.point(), hit.normal() + unit_sphere_random_point());
    0.5 * ray_color(&bounce_ray, world, bounce_depth - 1)
  } else {
    let direction = ray.direction().unit();
    let time = math::map_range(direction.y(), Range::new(-1.0, 1.0), Range::new(0.0, 1.0));

    color::linear_blend(COLOR_LIGHT_BLUE, COLOR_WHITE, time)
  }
}

fn main() {
  const MAX_BOUNCES: usize = 50;
  const SAMPLES_PER_PIXEL: usize = 100;
  const ASPECT_RATIO: f64 = 16.0 / 9.0;

  const IMAGE_WIDTH: usize = 400;
  const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

  const LAST_PIXEL_X: usize = IMAGE_WIDTH - 1;
  const LAST_PIXEL_Y: usize = IMAGE_HEIGHT - 1;

  let camera = Camera::new(ASPECT_RATIO);

  let world: Vec<Box<dyn Hittable>> = vec![
    Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
    Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
  ];

  // Render
  println!("P3");
  println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
  println!("255");

  for row in (0..IMAGE_HEIGHT).rev() {
    eprint!("\rScanlines remaining: {row}    ");

    for col in 0..IMAGE_WIDTH {
      let mut pixel_color = Color::default();

      for _ in 0..SAMPLES_PER_PIXEL {
        let u = (col as f64 + random::random()) / LAST_PIXEL_X as f64;
        let v = (row as f64 + random::random()) / LAST_PIXEL_Y as f64;

        let ray = camera.get_ray((u, v));

        pixel_color += ray_color(&ray, &world, MAX_BOUNCES);
      }

      color::print(pixel_color, SAMPLES_PER_PIXEL);
    }
  }

  eprintln!();
  eprintln!("Done.");
}
