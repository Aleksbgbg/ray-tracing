use crate::types::quadratic::Quadratic;
use crate::types::ray::Ray;
use crate::types::sphere::Sphere;
use crate::types::vec3::{Color, Point3, Vec3};
use crate::utils::color::{COLOR_LIGHT_BLUE, COLOR_WHITE};
use crate::utils::math::Range;
use crate::utils::{color, math};

mod types;
mod utils;

fn hit_sphere(ray: &Ray, sphere: &Sphere) -> Option<Vec3> {
  let ray_to_sphere = ray.origin() - sphere.center();

  let roots = Quadratic::new(
    ray.direction().length_squared(),
    2.0 * Vec3::dot(&ray.direction(), &ray_to_sphere),
    ray_to_sphere.length_squared() - math::square(sphere.radius()),
  )
  .find_real_roots();

  if roots.is_empty() {
    None
  } else {
    let hit_time = roots[0];
    Some(ray.at(hit_time))
  }
}

fn ray_color(ray: &Ray) -> Color {
  let sphere = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);

  if let Some(point) = hit_sphere(ray, &sphere) {
    let normal = (point - sphere.center()).unit();
    normal.map(|value| math::map_range(value, Range::new(-1.0, 1.0), Range::new(0.0, 1.0)))
  } else {
    let direction = ray.direction().unit();
    let time = math::map_range(direction.y(), Range::new(-1.0, 1.0), Range::new(0.0, 1.0));

    color::linear_blend(COLOR_LIGHT_BLUE, COLOR_WHITE, time)
  }
}

fn main() {
  // Image
  const ASPECT_RATIO: f64 = 16.0 / 9.0;

  const IMAGE_WIDTH: usize = 400;
  const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

  const LAST_PIXEL_X: usize = IMAGE_WIDTH - 1;
  const LAST_PIXEL_Y: usize = IMAGE_HEIGHT - 1;

  // Camera
  const VIEWPORT_HEIGHT: f64 = 2.0;
  const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
  const FOCAL_LENGTH: f64 = 1.0;

  let origin = Point3::default();
  const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
  const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

  let lower_left_corner =
    origin - (HORIZONTAL / 2.0) - (VERTICAL / 2.0) - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

  // Render
  println!("P3");
  println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
  println!("255");

  for row in (0..IMAGE_HEIGHT).rev() {
    eprint!("\rScanlines remaining: {row}    ");

    for col in 0..IMAGE_WIDTH {
      let u = col as f64 / LAST_PIXEL_X as f64;
      let v = row as f64 / LAST_PIXEL_Y as f64;

      let ray = Ray::new(
        origin,
        lower_left_corner + (u * HORIZONTAL) + (v * VERTICAL) - origin,
      );

      color::print(ray_color(&ray));
    }
  }

  eprintln!();
  eprintln!("Done.");
}
