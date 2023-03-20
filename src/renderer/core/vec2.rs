#[derive(Default, Clone, Copy)]
pub struct Vec2<T> {
  x: T,
  y: T,
}

impl<T> Vec2<T> {
  pub fn new(x: T, y: T) -> Self {
    Self { x, y }
  }
}

impl<T: Copy> Vec2<T> {
  pub fn x(&self) -> T {
    self.x
  }

  pub fn y(&self) -> T {
    self.y
  }
}
