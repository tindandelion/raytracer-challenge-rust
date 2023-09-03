use std::ops::Deref;

use crate::geometry::{Matrix, Point, Ray, UnitVector, Vector};

pub trait Transformable<T> {
    fn apply(&self, t: &Transform) -> T;
}

pub struct Transform {
    forward: Matrix,
    inverse: Matrix,
}

impl Transform {
    pub const IDENTITY: Transform = Transform {
        forward: Matrix::IDENTITY,
        inverse: Matrix::IDENTITY,
    };

    pub fn rotate_x(angle: f64) -> Transform {
        Transform {
            forward: Matrix::rotate_x(angle),
            inverse: Matrix::rotate_x(-angle),
        }
    }

    pub fn translate(vector: &Vector) -> Transform {
        Transform {
            forward: Matrix::translation(&vector),
            inverse: Matrix::translation(&-vector),
        }
    }

    pub fn inverse(&self) -> Transform {
        Transform {
            forward: self.inverse.clone(),
            inverse: self.forward.clone(),
        }
    }

    pub fn apply<U, T: Transformable<U>>(&self, arg: &T) -> U {
        arg.apply(self)
    }
}

impl Transformable<Vector> for Vector {
    fn apply(&self, t: &Transform) -> Vector {
        &t.forward * self
    }
}

impl Transformable<Vector> for UnitVector {
    fn apply(&self, t: &Transform) -> Vector {
        &t.forward * self.deref()
    }
}

impl Transformable<Ray> for Ray {
    fn apply(&self, t: &Transform) -> Ray {
        self.transform(&t.forward)
    }
}

impl Transformable<Point> for Point {
    fn apply(&self, t: &Transform) -> Point {
        &t.forward * self
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::geometry::{Point, Vector};

    use super::Transform;

    #[test]
    fn apply_rotation() {
        let original = Vector(0., 1., 0.);
        let transform = Transform::rotate_x(PI / 4.);

        let rotated = transform.apply(&original);
        let restored = transform.inverse().apply(&rotated);

        assert_eq!(rotated, Vector(0., SQRT_2 / 2., SQRT_2 / 2.));
        assert_eq!(restored, original);
    }

    #[test]
    fn apply_translation() {
        let original = Point::ZERO;
        let transform = Transform::translate(&Vector(1., 2., 3.));

        let translated = transform.apply(&original);
        let restored = transform.inverse().apply(&translated);

        assert_eq!(translated, Point::new(1., 2., 3.));
        assert_eq!(restored, original)
    }
}
