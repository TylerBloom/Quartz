
pub use super::spatial_vector::SpatialVector;

use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug, Clone, Copy)]
pub struct F32Vector3D {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl F32Vector3D {
    pub fn new() -> Self {
        F32Vector3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl fmt::Display for F32Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "<{}, {}, {}>", self.x, self.y, self.z )
    }
}

impl ops::Add for F32Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        F32Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for F32Vector3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        F32Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for F32Vector3D {
    type Output = f32;
    fn mul(self, rhs: Self) -> f32 {
        self.dot(&rhs)
    }
}

impl ops::Rem for F32Vector3D {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        self.cross(&rhs)
    }
}

impl cmp::PartialEq for F32Vector3D {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z)
    }
}

impl SpatialVector<f32> for F32Vector3D {
    fn dot(&self, rhs: &Self) -> f32 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
    
    fn cross(&self, rhs: &Self) -> Self {
        F32Vector3D {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }
    
    // Manual scalar multiplication and division
    fn scale(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
    
    // Calculates the vector norm
    fn length(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    
    // Normalizes the vector
    fn normalize(&mut self) {
        self.scale( 1.0/self.length() );
    }
}


#[derive(Debug, Clone, Copy)]
pub struct F64Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl F64Vector3D {
    pub fn new() -> Self {
        F64Vector3D { x: 0.0, y: 0.0, z: 0.0 }
    }
}

impl fmt::Display for F64Vector3D {
    fn fmt(&self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!( f, "<{}, {}, {}>", self.x, self.y, self.z )
    }
}

impl ops::Add for F64Vector3D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        F64Vector3D {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub for F64Vector3D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        F64Vector3D {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::Mul for F64Vector3D {
    type Output = f64;
    fn mul(self, rhs: Self) -> f64 {
        self.dot(&rhs)
    }
}

impl ops::Rem for F64Vector3D {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        self.cross(&rhs)
    }
}

impl cmp::PartialEq for F64Vector3D {
    fn eq(&self, rhs: &Self) -> bool {
        (self.x == rhs.x) && (self.y == rhs.y) && (self.z == rhs.z)
    }
}

impl SpatialVector<f64> for F64Vector3D {
    fn dot(&self, rhs: &Self) -> f64 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }
    
    fn cross(&self, rhs: &Self) -> Self {
        F64Vector3D {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }
    
    // Manual scalar multiplication and division
    fn scale(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
    
    // Calculates the vector norm
    fn length(&self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    
    // Normalizes the vector
    fn normalize(&mut self) {
        self.scale( 1.0/self.length() );
    }
}

