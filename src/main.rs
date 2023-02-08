use crate::types::vec3::Color;
use crate::utils::print_color;

mod types;
mod utils;

fn main() {
  const IMAGE_WIDTH: usize = 256;
  const IMAGE_HEIGHT: usize = 256;

  const LAST_PIXEL_X: usize = IMAGE_WIDTH - 1;
  const LAST_PIXEL_Y: usize = IMAGE_HEIGHT - 1;

  println!("P3");
  println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
  println!("255");

  for row in (0..IMAGE_HEIGHT).rev() {
    eprint!("\rScanlines remaining: {row}    ");

    for col in 0..IMAGE_WIDTH {
      let red = col as f64 / LAST_PIXEL_X as f64;
      let green = row as f64 / LAST_PIXEL_Y as f64;
      let blue = 0.50;

      print_color::print_color(&Color::new(red, green, blue));
    }
  }

  eprintln!();
  eprintln!("Done.");
}
