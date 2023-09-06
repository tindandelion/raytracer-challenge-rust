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

    pub fn rotate_y(angle: f64) -> Transform {
        Transform {
            forward: Matrix::rotate_y(angle),
            inverse: Matrix::rotate_y(-angle),
        }
    }

    pub fn translate(tx: f64, ty: f64, tz: f64) -> Transform {
        let v = Vector(tx, ty, tz);
        Transform {
            forward: Matrix::translation(&v),
            inverse: Matrix::translation(&v.flip()),
        }
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Transform {
        let v = Vector(sx, sy, sz);
        Transform {
            forward: Matrix::diag(&v),
            inverse: Matrix::diag(&v.recip()),
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

    pub fn apply<R, Obj: MatMul<R>>(&self, object: &Obj) -> R {
        object.matmul(&self.forward)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::geometry::{Point, Vector};

    use super::Transform;

    #[test]
    fn apply_rotation_around_x() {
        let original = Vector(0., 1., 0.);
        let transform = Transform::rotate_x(PI / 4.);

        let rotated = transform.apply(&original);
        let restored = transform.inverse().apply(&rotated);

        assert_eq!(rotated, Vector(0., SQRT_2 / 2., SQRT_2 / 2.));
        assert_eq!(restored, original);
    }

    #[test]
    fn apply_rotation_around_y() {
        let original = Vector(0., 0., 1.);
        let transform = Transform::rotate_y(PI / 4.);

        let rotated = transform.apply(&original);
        let restored = transform.inverse().apply(&rotated);

        assert_eq!(rotated, Vector(SQRT_2 / 2., 0., SQRT_2 / 2.));
        assert_eq!(restored, original);
    }

    #[test]
    fn apply_translation() {
        let original = Point::ZERO;
        let transform = Transform::translate(1., 2., 3.);

        let translated = transform.apply(&original);
        let restored = transform.inverse().apply(&translated);

        assert_eq!(translated, Point::new(1., 2., 3.));
        assert_eq!(restored, original)
    }

    #[test]
    fn apply_scaling() {
        let original = Vector(1., 2., 3.);
        let transform = Transform::scale(2., 3., 4.);

        let scaled = transform.apply(&original);
        let restored = transform.inverse().apply(&scaled);

        assert_eq!(scaled, Vector(2., 6., 12.));
        assert_eq!(restored, original);
    }

    mod compose_transforms {
        use super::*;

        #[test]
        fn translate_and_scale() {
            let original = Point::new(3., 2., 0.);
            let transform =
                Transform::translate(1., 1., 0.).and_then(&Transform::scale(0.5, 0.5, 1.));

            let transformed = transform.apply(&original);
            assert_eq!(transformed, Point::new(2., 1.5, 0.))
        }

        #[test]
        fn scale_and_translate() {
            let original = Point::new(3., 2., 0.);
            let transform =
                Transform::scale(0.5, 0.5, 1.).and_then(&Transform::translate(1., 1., 0.));

            let transformed = transform.apply(&original);
            assert_eq!(transformed, Point::new(2.5, 2., 0.))
        }

        #[test]
        fn rotate_around_point() {
            let transform = Transform::translate(-1., -1., -1.)
                .and_then(&Transform::rotate_x(PI / 2.))
                .and_then(&Transform::translate(1., 1., 1.));

            let original = Point::new(1., 2., 3.);

            let transformed = transform.apply(&original);
            let restored = transform.inverse().apply(&transformed);

            assert_eq!(transformed, Point::new(1., -1., 2.));
            assert_eq!(restored, original);
        }
    }
}
