pub use super::scalar::Scalar;
pub use super::spatial_vector::SpatialVector;

use std::cmp;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3D<S>
where
    S: Scalar,
{
    pub x: S,
    pub y: S,
    pub z: S,
}

impl<S> Vector3D<S>
where
    S: Scalar,
{
    #[allow(non_snake_case)]
    pub fn new(X: S, Y: S, Z: S) -> Self {
        Vector3D { x: X, y: Y, z: Z }
    }
}

impl<S> fmt::Display for Vector3D<S>
where
    S: Scalar,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl<S> ops::Add for Vector3D<S>
where
    S: Scalar,
{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<S> ops::Sub for Vector3D<S>
where
    S: Scalar,
{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<S> ops::Mul for Vector3D<S>
where
    S: Scalar,
{
    type Output = S;
    fn mul(self, rhs: Self) -> S {
        self.dot(&rhs)
    }
}

impl<S> ops::Mul<S> for Vector3D<S>
where
    S: Scalar,
{
    type Output = Self;
    fn mul(self, rhs: S) -> Self {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<S> ops::Rem for Vector3D<S>
where
    S: Scalar,
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        self.cross(&rhs)
    }
}

impl<S> cmp::PartialEq for Vector3D<S>
where
    S: Scalar,
{
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z)
    }
}

impl<S> SpatialVector<S> for Vector3D<S>
where
    S: Scalar,
{
    fn zero() -> Self {
        Vector3D {
            x: S::zero(),
            y: S::zero(),
            z: S::zero(),
        }
    }

    fn dot(&self, rhs: &Self) -> S {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    fn cross(&self, rhs: &Self) -> Self {
        Vector3D {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }

    // Manual scalar multiplication and division
    fn scale(&mut self, rhs: S) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }

    // Calculates the vector norm
    fn length(&self) -> S {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Normalizes the vector
    fn normalize(&mut self) {
        self.scale(self.length().inv());
    }
}
