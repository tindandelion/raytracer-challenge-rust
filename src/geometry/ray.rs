use super::{MatMul, Matrix, Point, UnitVector, Vector};

#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub const fn new(origin: Point, direction: UnitVector) -> Ray {
        Ray {
            origin,
            direction: direction.v(),
        }
    }

    pub fn between(origin: &Point, dest: &Point) -> Ray {
        let direction = (dest - origin).normalize();
        Self::new(origin.clone(), direction)
    }

    pub fn position(&self, distance: f64) -> Point {
        &self.origin + &self.direction * distance
    }
}

impl MatMul<Ray> for Ray {
    fn matmul(&self, m: &Matrix) -> Ray {
        Ray {
            origin: m * &self.origin,
            direction: m * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use super::*;

    #[test]
    fn calculate_ray_direction_from_origin_to_destination() {
        let origin = Point::ZERO;
        let destination = Point::new(1., 1., 0.);
        let ray = Ray::between(&origin, &destination);
        assert_eq!(ray.direction, Vector(SQRT_2 / 2., SQRT_2 / 2., 0.))
    }

    #[test]
    fn compute_point_from_distance() {
        let ray = Ray::new(Point::new(2., 3., 4.), Vector(1., 0., 0.).normalize());

        assert_eq!(Point::new(2., 3., 4.), ray.position(0.));
        assert_eq!(Point::new(3., 3., 4.), ray.position(1.));
        assert_eq!(Point::new(1., 3., 4.), ray.position(-1.));
        assert_eq!(Point::new(4.5, 3., 4.), ray.position(2.5));
    }

    #[test]
    fn translate_ray() {
        let ray = Ray::new(Point::new(1., 2., 3.), UnitVector::Y);
        let m = Matrix::translation(&Vector(3., 4., 5.));

        let transformed = ray.matmul(&m);
        assert_eq!(transformed.origin, Point::new(4., 6., 8.));
        assert_eq!(transformed.direction, UnitVector::Y);
    }

    #[test]
    fn rotate_ray() {
        let ray = Ray::new(Point::new(1., 2., 3.), UnitVector::Y);
        let m = Matrix::rotate_x(PI / 2.);

        let transformed = ray.matmul(&m);
        assert_eq!(transformed.origin, Point::new(1., -3., 2.));
        assert_eq!(transformed.direction, UnitVector::Z);
    }

    #[test]
    fn scale_ray_creates_non_unit_direction() {
        let ray = Ray::new(Point::new(1., 2., 3.), UnitVector::Y);
        let m = Matrix::diag(&Vector(2., 3., 4.));

        let transformed = ray.matmul(&m);
        assert_eq!(transformed.origin, Point::new(2., 6., 12.));
        assert_eq!(transformed.direction, Vector(0., 3., 0.))
    }
}
