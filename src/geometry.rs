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

#[cfg(test)]
mod tests {

    mod operations {
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
    }
}
