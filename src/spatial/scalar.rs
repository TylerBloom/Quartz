use std::cmp;
use std::fmt;
use std::ops;

pub trait Scalar
where
    Self: Sized,
    Self: Copy,
    Self: fmt::Display,
    Self: ops::Add<Output = Self>,
    Self: ops::Sub<Output = Self>,
    Self: ops::Mul<Output = Self>,
    Self: ops::Div<Output = Self>,
    Self: ops::MulAssign,
    Self: cmp::PartialOrd,
    Self: cmp::PartialEq,
{
    fn sqrt(self) -> Self;
    fn inv(self) -> Self;
}

impl Scalar for f32 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn inv(self) -> Self {
        1.0_f32 / self
    }
}

impl Scalar for f64 {
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn inv(self) -> Self {
        1.0_f64 / self
    }
}
