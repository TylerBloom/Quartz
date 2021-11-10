
use std::fmt;
use std::ops;
use std::cmp;

pub trait Scalar
    where Self: Sized,
          Self: fmt::Display,
          Self: ops::Add,
          Self: ops::Sub,
          Self: ops::Mul,
          Self: ops::Rem, // The cross ("outer") product
          Self: cmp::PartialOrd,
          Self: cmp::PartialEq {
}

impl Scalar for f32 {
}

impl Scalar for f64 {
}
