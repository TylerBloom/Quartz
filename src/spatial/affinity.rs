
pub use crate::math::scalar::Scalar;
pub use crate::math::lin_alg::spatial_vector::SpatialVector;
pub use super::position::Position;
pub use super::direction::Direction;

use std::cmp;
use std::fmt;
use std::ops;


pub trait Affinity<S, V>
where S: Scalar,
      V: SpatialVector<S>,
      Self: Sized,
      Self: fmt::Display,
      Self: cmp::PartialEq,
      Self: ops::Mul<Self,Output=Self>,
{
    fn new() -> Self;
    fn get_as_local_position(&self, pos: Position<S,V>) -> Position<S,V>;
    fn get_as_local_direction(&self, dir: Direction<S,V>) -> Direction<S,V>;
}
