
#[cfg(test)]
mod tests {
    use quartz::spatial::vector_3d::{SpatialVector, F64Vector3D};
    
    fn approx_eq( v1: &F64Vector3D, v2: &F64Vector3D, thres: f64 ) -> bool {
        let mut digest = true;
        println!( "{}", v1 );
        println!( "{}", v2 );
        if v1.x == 0.0 {
            digest &= v2.x == 0.0;
        } else {
            digest &= ((v1.x - v2.x)/(v1.x)).abs() <= thres;
            if ! digest {
                println!( "Bad X: {}", ((v1.x - v2.x)/(v1.x)).abs() );
            }
        }
        if v1.y == 0.0 {
            digest &= v2.y == 0.0;
        } else {
            digest &= ((v1.y - v2.y)/(v1.y)).abs() <= thres;
            if ! digest {
                println!( "Bad Y: {}", ((v1.y - v2.y)/(v1.y)).abs() );
            }
        }
        if v1.z == 0.0 {
            digest &= v2.z == 0.0;
        } else {
            digest &= ((v1.z - v2.z)/(v1.z)).abs() <= thres;
            if ! digest {
                println!( "Bad Z: {}", ((v1.z - v2.z)/(v1.z)).abs() );
            }
        }
        digest
    }
    
    #[test]
    fn create_f64_vector() {
        let _v1 = F64Vector3D::new();
        let _v2 = F64Vector3D { x: 1.0, y: 1.0, z: 1.0 };
    }
    
    #[test]
    fn populate_f64_vector() {
        let mut v1 = F64Vector3D::new();
        v1.x = 1.0;
        v1.y = 2.0;
        v1.z = 3.0;
        
        let mut v2 = F64Vector3D { x: 9.0, y: 9.0, z: 9.0 };
        v2.x = 1.0;
        v2.y = 2.0;
        v2.z = 3.0;
    }
    
    #[test]
    fn scale_f64_vector() {
        let mut v = F64Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        v.scale( 5.0 );
        assert_eq!( v, F64Vector3D { x: 5.0, y: 10.0, z: 15.0 } );
    }
    
    #[test]
    fn dot_f64_vectors() {
        let v1 = F64Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = F64Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        assert_eq!( v1*v2, 0.0 );
        let v3 = F64Vector3D { x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!( v1*v3, 6.0 );
        let v4 = F64Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!( v1*v4, 14.0 );
    }
    
    #[test]
    fn cross_f64_vectors() {
        let v1 = F64Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        let v2 = F64Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        assert_eq!( v1%v2, F64Vector3D::new() );
        let v3 = F64Vector3D { x: 1.0, y: 0.0, z: 0.0 };
        let v4 = F64Vector3D { x: 0.0, y: 1.0, z: 0.0 };
        assert_eq!( v3%v4, F64Vector3D { x: 0.0, y: 0.0, z: 1.0 } );
        let v5 = F64Vector3D { x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!( v1%v5, F64Vector3D { x: -1.0, y: 2.0, z: -1.0 } );
    }
    
    #[test]
    fn length_f64_vectors() {
        let v1 = F64Vector3D { x: 0.0, y: 0.0, z: 0.0 };
        assert_eq!( v1.length(), 0.0 );
        let v2 = F64Vector3D { x: 1.0, y: 0.0, z: 0.0 };
        assert_eq!( v2.length(), 1.0 );
        let v3 = F64Vector3D { x: 1.0, y: 1.0, z: 1.0 };
        assert_eq!( v3.length(), 3.0_f64.sqrt() );
        let v4 = F64Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        assert_eq!( v4.length(), 14.0_f64.sqrt() );
    }
    
    #[test]
    fn normalize_f64_vectors() {
        let mut v1 = F64Vector3D { x: 1.0, y: 0.0, z: 0.0 };
        v1.normalize();
        assert_eq!( v1, F64Vector3D { x: 1.0, y: 0.0, z: 0.0 } );
        let mut v2 = F64Vector3D { x: 1.0, y: 1.0, z: 1.0 };
        v2.normalize();
        assert!( approx_eq( &v2, &F64Vector3D { x: (1.0/3.0_f64).sqrt(), y: (1.0/3.0_f64).sqrt(), z: (1.0/3.0_f64).sqrt() }, 0.000000001) );
        let mut v3 = F64Vector3D { x: 1.0, y: 2.0, z: 3.0 };
        v3.normalize();
        assert!( approx_eq( &v3, &F64Vector3D { x: 1.0/14.0_f64.sqrt(), y: 2.0/14.0_f64.sqrt(), z: 3.0/14.0_f64.sqrt() }, 0.0000000001) );
    }
}
