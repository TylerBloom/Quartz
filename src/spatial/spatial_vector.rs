
use super::scalar::Scalar;

use std::fmt;
use std::ops;
use std::cmp;

pub trait SpatialVector<T> 
    where Self: Sized,
          Self: fmt::Display,
          Self: ops::Add,
          Self: ops::Sub,
          Self: ops::Rem, // The cross ("outer") product
          Self: cmp::PartialEq,
          T: Scalar {
    
    // The dot ("inner") product
    fn dot(&self, rhs: &Self) -> T;
    
    // Manual scalar multiplication and division
    fn scale(&mut self, rhs: T) -> ();
    
    // Calculates the vector norm
    fn length(&self) -> T;
    
    // Normalizes the vector
    fn normalize(&mut self) -> ();
}
