use std::ops::Mul;

pub fn square<T>(value: T) -> T
where
  T: Copy + Mul<Output = T>,
{
  value * value
}
