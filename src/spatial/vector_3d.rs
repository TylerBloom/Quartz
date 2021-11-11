pub use super::scalar::Scalar;
pub use super::spatial_vector::SpatialVector;

use std::cmp;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3D<T>
where
    T: Scalar,
{
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T>
where
    T: Scalar,
{
    #[allow(non_snake_case)]
    pub fn new(X: T, Y: T, Z: T) -> Self {
        Vector3D { x: X, y: Y, z: Z }
    }
}

impl<T> fmt::Display for Vector3D<T>
where
    T: Scalar,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<{}, {}, {}>", self.x, self.y, self.z)
    }
}

impl<T> ops::Add for Vector3D<T>
where
    T: Scalar,
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

impl<T> ops::Sub for Vector3D<T>
where
    T: Scalar,
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

impl<T> ops::Mul for Vector3D<T>
where
    T: Scalar,
{
    type Output = T;
    fn mul(self, rhs: Self) -> T {
        self.dot(&rhs)
    }
}

impl<T> ops::Mul<T> for Vector3D<T>
where
    T: Scalar,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self {
        Vector3D {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T> ops::Rem for Vector3D<T>
where
    T: Scalar,
{
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        self.cross(&rhs)
    }
}

impl<T> cmp::PartialEq for Vector3D<T>
where
    T: Scalar,
{
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z)
    }
}

impl<T> SpatialVector<T> for Vector3D<T>
where
    T: Scalar,
{
    fn dot(&self, rhs: &Self) -> T {
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
    fn scale(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }

    // Calculates the vector norm
    fn length(&self) -> T {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Normalizes the vector
    fn normalize(&mut self) {
        self.scale(self.length().inv());
    }
}
