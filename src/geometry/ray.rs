use std::ops::Deref;

use super::{Point, UnitVector, Vector};

pub struct Ray {
    pub origin: Point,
    pub direction: UnitVector,
}

impl Ray {
    pub const fn new(origin: Point, direction: UnitVector) -> Ray {
        Ray { origin, direction }
    }

    pub fn between(origin: &Point, dest: &Point) -> Ray {
        let direction = (dest - origin).normalize();
        Self::new(origin.clone(), direction)
    }

    pub fn position(&self, distance: f64) -> Point {
        &self.origin + self.direction.deref() * distance
    }

    pub fn scalar_projection_of(&self, v: &Vector) -> f64 {
        self.direction.dot(v)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use super::*;

    #[test]
    fn calculate_ray_direction_from_origin_to_destination() {
        let origin = Point::ZERO;
        let destination = Point::new(1., 1., 0.);
        let ray = Ray::between(&origin, &destination);
        assert_eq!(*ray.direction, Vector(SQRT_2 / 2., SQRT_2 / 2., 0.))
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
    fn compute_vector_projection() {
        let ray = Ray::new(Point::new(2., 3., 4.), Vector(1., 0., 0.).normalize());

        assert_eq!(
            ray.scalar_projection_of(&Vector(1., 1., 0.)),
            1.,
            "Arbitrary vector"
        );
        assert_eq!(
            ray.scalar_projection_of(&Vector(0., 1., 0.)),
            0.,
            "Orthogonal vector"
        );
        assert_eq!(
            ray.scalar_projection_of(&Vector(-1., 0., 0.)),
            -1.,
            "Opposite direction vector"
        );
    }
}