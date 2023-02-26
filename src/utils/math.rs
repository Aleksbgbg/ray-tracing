use std::ops::{Add, Div, Mul, Sub};

pub trait Mathematical:
  Copy
  + PartialOrd
  + Add<Output = Self>
  + Sub<Output = Self>
  + Mul<Output = Self>
  + Div<Output = Self>
{
}

impl Mathematical for f64 {}

pub fn square<T: Mathematical>(value: T) -> T {
  value * value
}

/// A range [start, end).
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
  debug_assert!(val >= src.start);
  debug_assert!(val < src.end);

  ((val - src.start) * (dst.dist() / src.dist())) + dst.start
}
