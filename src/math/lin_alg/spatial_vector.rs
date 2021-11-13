use crate::math::scalar::Scalar;

use std::cmp;
use std::fmt;
use std::ops;

pub trait SpatialVector<S>
where
    Self: Sized,
    Self: fmt::Display,
    Self: ops::Add<Self, Output = Self>,
    Self: ops::Sub<Self, Output = Self>,
    Self: ops::Mul<Self, Output = S>, // The dot ("inner") product
    Self: ops::Mul<S, Output = Self>, // Scalar multiplication
    Self: ops::Rem<Output = Self>,    // The cross ("outer") product
    Self: cmp::PartialEq,
    S: Scalar,
{
    // Outputs dimensionality of the vector
    fn size() -> usize;
    
    // The zero vector
    fn zero() -> Self;

    // The dot ("inner") product
    fn dot(&self, rhs: &Self) -> S;

    // The cross ("inner") product
    fn cross(&self, rhs: &Self) -> Self;

    // Manual scalar multiplication and division
    fn scale(&mut self, rhs: S) -> ();

    // Calculates the vector norm
    fn length(&self) -> S;

    // Normalizes the vector
    fn normalize(&mut self) -> Result<(),&str>;
}
