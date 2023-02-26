use crate::types::vec3::Color;

pub const COLOR_WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const COLOR_RED: Color = Color::new(1.0, 0.0, 0.0);
pub const COLOR_LIGHT_BLUE: Color = Color::new(0.5, 0.7, 1.0);

pub fn linear_blend(first: Color, second: Color, fraction: f64) -> Color {
  (fraction * first) + ((1.0 - fraction) * second)
}

pub fn print(color: Color) {
  let (red, green, blue) = color.tuple();

  let red_scaled = 255f64 * red;
  let green_scaled = 255f64 * green;
  let blue_scaled = 255f64 * blue;

  println!("{red_scaled} {green_scaled} {blue_scaled}");
}
