use overload::overload;
use std::ops::{self, Deref};
use super::{UnitVector, Point, Vector};

#[derive(Debug, PartialEq, Clone)]
pub struct Matrix([f64; 16]);

pub trait MatMul<T> {
    fn matmul(&self, m: &Matrix) -> T;
}

overload!((a: ?Matrix) * (b: ?Matrix) -> Matrix { a.matmul(&b) });
overload!((a: ?Matrix) * (p: ?Point) -> Point { p.matmul(&a) });
overload!((a: ?Matrix) * (v: ?Vector) -> Vector { v.matmul(&a) });


impl Matrix {
    pub const IDENTITY: Matrix = Matrix([
        1., 0., 0., 0., 
        0., 1., 0., 0., 
        0., 0., 1., 0., 
        0., 0., 0., 1.
    ]);

    pub fn new(values: [f64; 16]) -> Matrix {
        Matrix(values)
    }

    fn zero() -> Matrix  {
        Matrix([
            0., 0., 0., 0., 
            0., 0., 0., 0., 
            0., 0., 0., 0., 
            0., 0., 0., 0.
        ])
    }   

    pub fn translation(v: &Vector) -> Matrix {
        Matrix([
            1., 0., 0., v.0, 
            0., 1., 0., v.1, 
            0., 0., 1., v.2, 
            0., 0., 0., 1.
        ])
    }

    pub fn rotate_x(angle: f64) -> Matrix {
        Matrix([
            1.,          0.,           0., 0., 
            0., angle.cos(), -angle.sin(), 0.,
            0., angle.sin(),  angle.cos(), 0., 
            0.,          0.,           0., 1.
        ])
    }

    pub fn rotate_y(angle: f64) -> Matrix {
        Matrix([
             angle.cos(), 0., angle.sin(), 0.,
                      0., 1.,          0., 0., 
            -angle.sin(), 0., angle.cos(), 0., 
                      0., 0.,          0., 1.
        ])
    }

    pub fn from_vectors(x: &UnitVector, y: &UnitVector, z: &UnitVector) -> Matrix {
        #[rustfmt::skip]
        let matrix = [
            x.0, y.0, z.0, 0., 
            x.1, y.1, z.1, 0., 
            x.2, y.2, z.2, 0., 
            0.,  0.,  0.,  1.,
        ];
        Matrix(matrix)
    }

    pub fn mul<R, T: MatMul<R>>(&self, arg: &T) -> R {
        arg.matmul(self)
    }

    fn el(&self, row: usize, col: usize) -> f64 {
        self.0[row * 4 + col]
    }
}

impl MatMul<Matrix> for Matrix {
    fn matmul(&self, other: &Matrix) -> Matrix {
        let mut result = Matrix::zero();
        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    result.0[row * 4 + col] += self.el(row, i) * other.el(i, col)
                }
            } 
        }
        result
    }
}

impl MatMul<Point> for Point {
    fn matmul(&self, m: &Matrix) -> Point {
        let v = self.as_vector();
        Point::new(
            v.0 * m.el(0, 0) + v.1 * m.el(0, 1) + v.2 * m.el(0, 2) + m.el(0, 3),
            v.0 * m.el(1, 0) + v.1 * m.el(1, 1) + v.2 * m.el(1, 2) + m.el(1, 3),
            v.0 * m.el(2, 0) + v.1 * m.el(2, 1) + v.2 * m.el(2, 2) + m.el(2, 3),
        )
    }
}

impl MatMul<Vector> for Vector {
    fn matmul(&self, m: &Matrix) -> Vector {
        Vector(
            self.0 * m.el(0, 0) + self.1 * m.el(0, 1) + self.2 * m.el(0, 2),
            self.0 * m.el(1, 0) + self.1 * m.el(1, 1) + self.2 * m.el(1, 2),
            self.0 * m.el(2, 0) + self.1 * m.el(2, 1) + self.2 * m.el(2, 2),
        )
    }
}

impl MatMul<Vector> for UnitVector {
    fn matmul(&self, m: &Matrix) -> Vector {
        self.deref().matmul(m)
    }
}

#[cfg(test)]
mod transform_tests {
    use crate::geometry::{Vector, Point, Matrix};


    #[test]
    fn translate_point() {
        let transform = Matrix::translation(&Vector(5., -3., 2.));
        let p = Point::new(-3., 4., 5.);
        assert_eq!(transform * p, Point::new(2., 1., 7.))
    }

    #[test]
    fn translate_vector_does_not_affect_it() {
        let transform = Matrix::translation(&Vector(5., -3., 2.));
        let v = Vector(-3., 4., 5.);
        assert_eq!(transform * &v, v)
    }

}

#[cfg(test)]
mod matrix_operations {
    use super::Matrix;

    #[test]
    fn multiply_matrices() {
        let a = Matrix::new([
            1., 2., 3., 4., 
            5., 6., 7., 8., 
            9., 8., 7., 6., 
            5., 4., 3., 2.
        ]);
        let b = Matrix::new([
            -2., 1., 2.,  3., 
             3., 2., 1., -1., 
             4., 3., 6.,  5., 
             1., 2., 7.,  8.
        ]);
        
        assert_eq!(a * b, Matrix::new([
            20., 22.,  50.,  48., 
            44., 54., 114., 108., 
            40., 58., 110., 102.,
            16., 26.,  46.,  42.
        ]))
    }
}