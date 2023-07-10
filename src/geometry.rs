use overload::overload;
use std::ops;

#[derive(PartialEq, Debug)]
struct Vector(f64, f64, f64);
#[derive(PartialEq, Debug)]
struct Point(f64, f64, f64);

overload!(- (a: ?Vector) -> Vector { Vector(-a.0, -a.1, -a.2) });

overload!((a: ?Vector) + (b: ?Point) -> Point { Point(a.0 + b.0, a.1 + b.1, a.2 + b.2) });
overload!((a: ?Vector) + (b: ?Vector) -> Vector { Vector(a.0 + b.0, a.1 + b.1, a.2 + b.2) });
overload!((a: ?Point) + (b: ?Vector) -> Point { b + a });

overload!((a: ?Point) - (b: ?Point) -> Vector { Vector(a.0 - b.0, a.1 - b.1, a.2 - b.2)});
overload!((a: ?Point) - (b: ?Vector) -> Point { -b + a });
overload!((a: ?Vector) - (b: ?Vector) -> Vector { -b + a });

overload!((v: ?Vector) * (c: f64) -> Vector { Vector(v.0 *c, v.1 * c, v.2 * c)});

impl Vector {
    fn magnitude(&self) -> f64 {
        self.dot(self).sqrt()
    }

    fn normalize(&self) -> Vector {
        self * (1. / self.magnitude())
    }

    fn dot(&self, v: &Vector) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    fn cross(&self, v: &Vector) -> Vector {
        Vector(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::*;

    #[test]
    fn add_vector_and_point() {
        let left = Point(3., -2., 5.);
        let right = Vector(-2., 3., 1.);

        let sum_left = &left + &right;
        let sum_right = &right + &left;

        assert_eq!(sum_left, Point(1., 1., 6.));
        assert_eq!(sum_left, sum_right);
    }

    #[test]
    fn add_vector_and_vector() {
        let left = Vector(3., -2., 5.);
        let right = Vector(-2., 3., 1.);

        let sum_left = &left + &right;
        let sum_right = &right + &left;

        assert_eq!(sum_left, Vector(1., 1., 6.));
        assert_eq!(sum_left, sum_right);
    }

    #[test]
    fn negate_vector() {
        let source = Vector(1., -2., 3.);

        let result = -source;
        assert_eq!(result, Vector(-1., 2., -3.))
    }

    #[test]
    fn subtract_point_from_point() {
        let left = Point(3., 2., 1.);
        let right = Point(5., 6., 7.);

        let result = left - right;
        assert_eq!(result, Vector(-2., -4., -6.))
    }

    #[test]
    fn subtract_vector_from_point() {
        let left = Point(3., 2., 1.);
        let right = Vector(5., 6., 7.);

        let result = left - right;
        assert_eq!(result, Point(-2., -4., -6.));
    }

    #[test]
    fn subtract_vector_from_vector() {
        let left = Vector(3., 2., 1.);
        let right = Vector(5., 6., 7.);

        let result = left - right;
        assert_eq!(result, Vector(-2., -4., -6.))
    }

    #[test]
    fn multiply_vector_by_scalar() {
        let vec = Vector(1., -2., 3.);
        assert_eq!(vec * 3.5, Vector(3.5, -7., 10.5))
    }

    #[test]
    fn vector_magnitude() {
        assert_eq!(Vector(1., 0., 0.).magnitude(), 1.);
        assert_eq!(Vector(0., 1., 0.).magnitude(), 1.);
        assert_eq!(Vector(0., 1., 0.).magnitude(), 1.);
        assert_eq!(Vector(1., 2., 3.).magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalize_vector() {
        assert_eq!(Vector(4., 0., 0.).normalize(), Vector(1., 0., 0.));
        assert_eq!(
            Vector(1., 2., 2.).normalize(),
            Vector(1. / 3., 2. / 3., 2. / 3.)
        )
    }

    #[test]
    fn vector_cross_product() {
        let a = Vector(1., 2., 3.);
        let b = Vector(2., 3., 4.);

        assert_eq!(a.cross(&b), Vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), Vector(1., -2., 1.));
    }
}
