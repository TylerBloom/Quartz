pub use super::direction::Direction;
pub use crate::math::scalar::Scalar;
pub use crate::math::lin_alg::spatial_vector::SpatialVector;

use std::cmp;
use std::fmt;
use std::marker::PhantomData;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    pub v: V,
    s: PhantomData<S>,
}

impl<S, V> Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    pub fn new(vec: V) -> Self {
        Position {
            v: vec,
            s: PhantomData,
        }
    }
}

impl<S, V> fmt::Display for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.v)
    }
}

impl<S, V> ops::Add for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Position::new(self.v + rhs.v)
    }
}

impl<S, V> ops::Sub for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Position::new(self.v - rhs.v)
    }
}

impl<S, V> ops::Mul for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = S;
    fn mul(self, rhs: Self) -> S {
        self.dot(&rhs)
    }
}

impl<S, V> ops::Mul<S> for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Self;
    fn mul(self, rhs: S) -> Self {
        Position::new(self.v * rhs)
    }
}

impl<S, V> ops::Rem for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        self.cross(&rhs)
    }
}

impl<S, V> cmp::PartialEq for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    fn eq(&self, rhs: &Self) -> bool {
        self.v == rhs.v
    }
}

impl<S, V> SpatialVector<S> for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    // We add one to the size for the homogeneous coordinate used in affine transforms
    fn size() -> usize {
        V::size() + 1
    }
    
    fn zero() -> Self {
        Position {
            v: V::zero(),
            s: PhantomData,
        }
    }

    fn dot(&self, rhs: &Self) -> S {
        self.v.dot(&rhs.v)
    }

    fn cross(&self, rhs: &Self) -> Self {
        Position::new(self.v.cross(&(rhs.v)))
    }

    fn scale(&mut self, rhs: S) -> () {
        self.v.scale(rhs)
    }

    fn length(&self) -> S {
        self.v.length()
    }

    fn normalize(&mut self) -> Result<(),&str> {
        self.v.normalize()
    }
}

impl<S, V> ops::Add<Position<S, V>> for Direction<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Position<S, V>;
    fn add(self, rhs: Position<S, V>) -> Position<S, V> {
        Position::new(self.v + rhs.v)
    }
}

impl<S, V> ops::Add<Direction<S, V>> for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Position<S, V>;
    fn add(self, rhs: Direction<S, V>) -> Position<S, V> {
        Position::new(self.v + rhs.v)
    }
}

impl<S, V> ops::Sub<Position<S, V>> for Direction<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Position<S, V>;
    fn sub(self, rhs: Position<S, V>) -> Position<S, V> {
        Position::new(self.v - rhs.v)
    }
}

impl<S, V> ops::Sub<Direction<S, V>> for Position<S, V>
where
    S: Scalar,
    V: SpatialVector<S>,
{
    type Output = Position<S, V>;
    fn sub(self, rhs: Direction<S, V>) -> Position<S, V> {
        Position::new(self.v - rhs.v)
    }
}
