use std::cmp;
use std::fmt;
use std::ops;

pub trait Scalar
where
    Self: Sized,
    Self: Copy,
    Self: fmt::Display,
    Self: ops::Add<Self, Output = Self>,
    Self: ops::Sub<Self, Output = Self>,
    Self: ops::Mul<Self, Output = Self>,
    Self: ops::Div<Self, Output = Self>,
    Self: ops::AddAssign,
    Self: ops::SubAssign,
    Self: ops::MulAssign,
    Self: ops::DivAssign,
    Self: cmp::PartialEq,
{
    fn from(a: f64) -> Self;
    fn sqrt(self) -> Self;
    fn inv(self) -> Self;
    fn zero() -> Self;
}

impl Scalar for f32 {
    fn from(a: f64) -> Self {
        a as f32
    }
    
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn inv(self) -> Self {
        1.0_f32 / self
    }

    fn zero() -> Self {
        0.0_f32
    }
}

impl Scalar for f64 {
    fn from(a: f64) -> Self {
        a
    }
    
    fn sqrt(self) -> Self {
        self.sqrt()
    }

    fn inv(self) -> Self {
        1.0_f64 / self
    }

    fn zero() -> Self {
        0.0_f64
    }
}
