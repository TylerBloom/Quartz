use super::scalar::Scalar;

use std::cmp;
use std::fmt;
use std::ops;

pub trait SpatialVector<T>
where
    Self: Sized,
    Self: fmt::Display,
    Self: ops::Add,
    Self: ops::Sub,
    Self: ops::Mul, // The dot ("inner") product
    Self: ops::Rem, // The cross ("outer") product
    Self: cmp::PartialEq,
    T: Scalar,
{
    // The dot ("inner") product
    fn dot(&self, rhs: &Self) -> T;

    // The cross ("inner") product
    fn cross(&self, rhs: &Self) -> Self;

    // Manual scalar multiplication and division
    fn scale(&mut self, rhs: T) -> ();

    // Calculates the vector norm
    fn length(&self) -> T;

    // Normalizes the vector
    fn normalize(&mut self) -> ();
}
