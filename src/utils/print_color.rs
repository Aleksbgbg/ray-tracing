use crate::types::vec3::Color;

pub fn print_color(color: &Color) {
  let (red, green, blue) = color.tuple();

  let red_scaled = 255f64 * red;
  let green_scaled = 255f64 * green;
  let blue_scaled = 255f64 * blue;

  println!("{red_scaled} {green_scaled} {blue_scaled}");
}
