use crate::renderer::core::vec3::Color;

pub const COLOR_WHITE: Color = Color::new(1.0, 1.0, 1.0);
pub const COLOR_LIGHT_BLUE: Color = Color::new(0.5, 0.7, 1.0);

pub fn linear_blend(first: Color, second: Color, fraction: f64) -> Color {
  (fraction * first) + ((1.0 - fraction) * second)
}

fn gamma_correct(value: f64) -> f64 {
  value.sqrt()
}

pub fn calculate_color(color: Color, samples: usize) -> Color {
  color.map(|component| 255.0 * gamma_correct(component / samples as f64))
}

pub fn print_color(color: Color) {
  let (red, green, blue) = color.tuple();
  println!("{} {} {}", red as usize, green as usize, blue as usize);
}
