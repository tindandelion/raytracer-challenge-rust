use crate::geometry::{MatMul, Matrix, Vector};

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

    pub fn and_then(&self, other: &Transform) -> Transform {
        Transform {
            forward: &other.forward * &self.forward,
            inverse: &self.inverse * &other.inverse,
        }
    }

    pub fn apply<U, T: MatMul<U>>(&self, arg: &T) -> U {
        arg.matmul(&self.forward)
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

    #[test]
    fn compose_transforms_rotate_around_point() {
        let rotation_point = Vector(1., 1., 1.);
        let transform = Transform::translate(&rotation_point.flip())
            .and_then(&Transform::rotate_x(PI / 2.))
            .and_then(&Transform::translate(&rotation_point));

        let original = Point::new(1., 2., 3.);

        let transformed = transform.apply(&original);
        let restored = transform.inverse().apply(&transformed);

        assert_eq!(transformed, Point::new(1., -1., 2.));
        assert_eq!(restored, original);
    }
}
