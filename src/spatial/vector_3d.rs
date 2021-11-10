
use super::spatial_vector::SpatialVector;

use std::fmt;
use std::ops;
use std::cmp;

#[derive(Debug)]
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

impl ops::Rem for F32Vector3D {
    type Output = Self;
    fn rem(self, rhs: Self) -> Self {
        F32Vector3D {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.z*rhs.y,
        }
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
    
    // Manual scalar multiplication and division
    fn scale(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
    
    // Calculates the vector norm
    fn length(&self) -> f32 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }
    
    // Normalizes the vector
    fn normalize(&mut self) {
        self.scale( self.length() );
    }
}


#[cfg(test)]
mod tests {
    use super::super::spatial_vector::SpatialVector;
    use super::F32Vector3D;
    
    #[test]
    fn create_vector() {
        let _v1 = F32Vector3D::new();
        let _v2 = F32Vector3D { x: 1.0, y: 1.0, z: 1.0 };
    }
    
    #[test]
    fn populate_vector() {
        let mut v1 = F32Vector3D::new();
        v1.x = 1.0;
        v1.y = 2.0;
        v1.z = 3.0;
        
        let mut v2 = F32Vector3D { x: 9.0, y: 9.0, z: 9.0 };
        v2.x = 1.0;
        v2.y = 2.0;
        v2.z = 3.0;
    }
    
    #[test]
    fn scale_vector() {
        let mut v = F32Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        v.scale( 5.0 );
        assert_eq!( v, F32Vector3D { x: 5.0, y: 10.0, z: 15.0 } );
    }
    
    #[test]
    fn dot_vectors() {
        let v1 = F32Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = F32Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        assert_eq!( v1.dot(&v2), 0.0 );
        let v3 = F32Vector3D { x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!( v1.dot(&v3), 6.0 );
        let v4 = F32Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!( v1.dot(&v4), 14.0 );
    }
}
