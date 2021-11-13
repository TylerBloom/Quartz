
use crate::math::scalar::Scalar;
use super::spatial_vector::SpatialVector;

use std::cmp;
use std::fmt;
use std::ops;
use std::marker::PhantomData;


pub trait Matrix<S, V>
where S: Scalar,
      V: SpatialVector<S>,
      Self: Sized,
      Self: fmt::Display,
      Self: cmp::PartialEq,
      Self: ops::Mul<Self,Output=Self>,
      Self: ops::Mul<V,Output=Self>,
{
    // Take a series of vectors that will be the rows matrix.
    // In the general case, this will never return an error, but specific
    // matrice, like SquareMatrix, will make use of the Err mode
    fn constuct_col_matrix(vecs: &Vec<V>) -> Result<Self,&str>;
    
    // Take a series of vectors that will be the rows of the matrix.
    // Same error info as before
    fn constuct_row_matrix(vecs: &Vec<V>) -> Result<Self,&str>;
    
    // Returns the dimensions of the matrix, (m,n)
    fn size(&self) -> (usize,usize);
}


pub struct GeneralMatrix<S, V>
where S: Scalar,
      V: SpatialVector<S>,
{
    _s: PhantomData<S>,
    matrix: Vec<V>,
}

impl<S,V> GeneralMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn new() -> Self {
        GeneralMatrix {
            _s: PhantomData,
            matrix: Vec::new()
        }
    }
}

impl<S,V> fmt::Display for GeneralMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut digest = String::new();
        for (pos, v) in self.matrix.iter().enumerate() {
            if pos == 0 {
                digest += &format!( "⎡ {} ⎤", v );
            } else if pos == self.matrix.len() {
                digest += &format!( "\n⎥ {} ⎥", v );
            } else {
                digest += &format!( "\n⎣ {} ⎦", v );
            }
        }
        write!( f, "{}", digest )
    }
}

impl<S,V> cmp::PartialEq for GeneralMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn eq(self: &GeneralMatrix<S,V>, rhs: &GeneralMatrix<S,V>) -> bool {
        self.matrix == rhs.matrix
    }
}

impl<S,V> ops::Mul<Self> for GeneralMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // TODO: Actually do this...
        GeneralMatrix::new()
    }
}

impl<S,V> ops::Mul<V> for GeneralMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    type Output = Self;
    fn mul(self, rhs: V) -> Self {
        // TODO: Actually do this...
        GeneralMatrix::new()
    }
}

impl<S,V> Matrix<S,V> for GeneralMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn constuct_col_matrix(vecs: &Vec<V>) -> Result<Self,&str> {
        // TODO: Actually do this...
        Ok( GeneralMatrix::new() )
    }
    
    fn constuct_row_matrix(vecs: &Vec<V>) -> Result<Self,&str> {
        // TODO: Actually do this...
        Ok( GeneralMatrix::new() )
    }
    
    fn size(&self) -> (usize,usize) {
        (self.matrix.len(),V::size())
    }
}

pub struct SquareMatrix<S, V> 
where S: Scalar,
      V: SpatialVector<S>,
{
    _s: PhantomData<S>,
    matrix: Vec<V>,
}

impl<S,V> SquareMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn new() -> Self {
        SquareMatrix {
            _s: PhantomData,
            matrix: Vec::new()
        }
    }
}

impl<S,V> fmt::Display for SquareMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut digest = String::new();
        for (pos, v) in self.matrix.iter().enumerate() {
            if pos == 0 {
                digest += &format!( "⎡ {} ⎤", v );
            } else if pos == self.matrix.len() {
                digest += &format!( "\n⎥ {} ⎥", v );
            } else {
                digest += &format!( "\n⎣ {} ⎦", v );
            }
        }
        write!( f, "{}", digest )
    }
}

impl<S,V> cmp::PartialEq for SquareMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn eq(self: &SquareMatrix<S,V>, rhs: &SquareMatrix<S,V>) -> bool {
        self.matrix == rhs.matrix
    }
}

impl<S,V> ops::Mul<Self> for SquareMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        // TODO: Actually do this...
        SquareMatrix::new()
    }
}

impl<S,V> ops::Mul<V> for SquareMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    type Output = Self;
    fn mul(self, rhs: V) -> Self {
        // TODO: Actually do this...
        SquareMatrix::new()
    }
}


impl<S,V> Matrix<S,V> for SquareMatrix<S,V>
where S: Scalar,
      V: SpatialVector<S>,
{
    fn constuct_col_matrix(vecs: &Vec<V>) -> Result<Self,&str> {
        // TODO: Actually do this...
        Ok( SquareMatrix::new() )
    }
    
    fn constuct_row_matrix(vecs: &Vec<V>) -> Result<Self,&str> {
        // TODO: Actually do this...
        Ok( SquareMatrix::new() )
    }
    
    fn size(&self) -> (usize,usize) {
        (self.matrix.len(),V::size())
    }
}
