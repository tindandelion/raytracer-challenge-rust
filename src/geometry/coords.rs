use overload::overload;
use std::ops::{self, Deref};

#[derive(Debug, Clone)]
pub struct Vector(pub f64, pub f64, pub f64);
#[derive(Debug, PartialEq, Clone)]
pub struct UnitVector(Vector);
#[derive(Debug, Clone)]
pub struct Point(Vector);

overload!(- (a: ?Vector) -> Vector { Vector(-a.0, -a.1, -a.2) });

overload!((a: ?Vector) + (b: ?Point) -> Point { Point::wrap_vector(a + &b.0) });
overload!((a: ?Vector) + (b: ?Vector) -> Vector { Vector(a.0 + b.0, a.1 + b.1, a.2 + b.2) });
overload!((a: ?Point) + (b: ?Vector) -> Point { b + a });

overload!((a: ?Point) - (b: ?Point) -> Vector { &a.0 - &b.0 });
overload!((a: ?Point) - (b: ?Vector) -> Point { -b + a });
overload!((a: ?Vector) - (b: ?Vector) -> Vector { -b + a });

overload!((v: ?Vector) * (c: f64) -> Vector { Vector(v.0 *c, v.1 * c, v.2 * c)});

const EQUALITY_TOLERANCE: f64 = 1e-6;

impl Point {
    pub const ZERO: Point = Point::new(0., 0., 0.);

    pub const fn new(x: f64, y: f64, z: f64) -> Point {
        Self::wrap_vector(Vector(x, y, z))
    }

    const fn wrap_vector(pv: Vector) -> Point {
        Point(pv)
    }

    pub fn x(&self) -> f64 {
        self.0 .0
    }

    pub fn y(&self) -> f64 {
        self.0 .1
    }
    pub fn z(&self) -> f64 {
        self.0 .2
    }
}

impl Vector {
    pub const ZERO: Vector = Vector(0., 0., 0.);

    pub fn magnitude_squared(&self) -> f64 {
        self.dot(self)
    }

    pub fn magnitude(&self) -> f64 {
        self.magnitude_squared().sqrt()
    }

    pub fn is_unit(&self) -> bool {
        (self.magnitude() - 1.0).abs() <= EQUALITY_TOLERANCE
    }

    pub fn normalize(&self) -> UnitVector {
        UnitVector::new(self * (1. / self.magnitude()))
    }

    pub fn dot(&self, v: &Vector) -> f64 {
        self.0 * v.0 + self.1 * v.1 + self.2 * v.2
    }

    pub fn cross(&self, v: &Vector) -> Vector {
        Vector(
            self.1 * v.2 - self.2 * v.1,
            self.2 * v.0 - self.0 * v.2,
            self.0 * v.1 - self.1 * v.0,
        )
    }

    pub fn recip(&self) -> Vector {
        Vector(self.0.recip(), self.1.recip(), self.2.recip())
    }

    pub fn flip(&self) -> Vector {
        Vector(-self.0, -self.1, -self.2)
    }

    pub fn is_approx_equal(&self, other: &Vector, tolerance: f64) -> bool {
        (self - other).magnitude() <= tolerance
    }
}

impl UnitVector {
    pub const X: UnitVector = UnitVector(Vector(1., 0., 0.));
    pub const Y: UnitVector = UnitVector(Vector(0., 1., 0.));
    pub const Z: UnitVector = UnitVector(Vector(0., 0., 1.));

    fn new(v: Vector) -> UnitVector {
        UnitVector(v)
    }

    pub fn flip(&self) -> UnitVector {
        UnitVector(-&self.0)
    }
}

impl Deref for UnitVector {
    type Target = Vector;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Point {
    pub fn as_vector(&self) -> &Vector {
        &self.0
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        self.is_approx_equal(other, EQUALITY_TOLERANCE)
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self - other).magnitude() < EQUALITY_TOLERANCE
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector_and_point() {
        let left = Point::new(3., -2., 5.);
        let right = Vector(-2., 3., 1.);

        let sum_left = &left + &right;
        let sum_right = &right + &left;

        assert_eq!(sum_left, Point::new(1., 1., 6.));
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
    fn subtract_point_from_point_makes_a_vector() {
        let dest = Point::new(3., 2., 1.);
        let src = Point::new(5., 6., 7.);

        let result = dest - src;
        assert_eq!(result, Vector(-2., -4., -6.))
    }

    #[test]
    fn subtract_vector_from_point() {
        let left = Point::new(3., 2., 1.);
        let right = Vector(5., 6., 7.);

        let result = left - right;
        assert_eq!(result, Point::new(-2., -4., -6.));
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
        assert_eq!(*Vector(4., 0., 0.).normalize(), Vector(1., 0., 0.));
        assert_eq!(
            *Vector(1., 2., 2.).normalize(),
            Vector(1. / 3., 2. / 3., 2. / 3.)
        );
    }

    #[test]
    fn test_if_vector_is_unit() {
        assert!(!Vector(4., 0., 0.).is_unit(), "Non-unit vector");
        assert!(Vector(1., 0., 0.).is_unit(), "Unit vector by default");
        assert!(
            Vector(1., 1., 0.).normalize().is_unit(),
            "Vector after normalization"
        );
    }

    #[test]
    fn vector_cross_product() {
        let a = Vector(1., 2., 3.);
        let b = Vector(2., 3., 4.);

        assert_eq!(a.cross(&b), Vector(-1., 2., -1.));
        assert_eq!(b.cross(&a), Vector(1., -2., 1.));
    }
}
