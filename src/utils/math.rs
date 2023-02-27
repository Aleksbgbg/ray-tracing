use std::ops::{Add, Div, Mul, Sub};

pub trait Mathematical:
  Copy
  + PartialOrd
  + Add<Output = Self>
  + Sub<Output = Self>
  + Mul<Output = Self>
  + Div<Output = Self>
{
  fn is_within(self, range: Range<Self>) -> bool {
    (range.start <= self) && (self < range.end)
  }
}

impl Mathematical for f64 {}

pub fn square<T: Mathematical>(value: T) -> T {
  value * value
}

/// A range [start, end).
#[derive(Clone, Copy)]
pub struct Range<T> {
  start: T,
  end: T,
}

impl<T: Mathematical> Range<T> {
  pub fn new(start: T, end: T) -> Range<T> {
    Range { start, end }
  }

  pub fn dist(&self) -> T {
    self.end - self.start
  }
}

pub fn map_range<T: Mathematical>(val: T, src: Range<T>, dst: Range<T>) -> T {
  debug_assert!(val.is_within(src));

  ((val - src.start) * (dst.dist() / src.dist())) + dst.start
}
