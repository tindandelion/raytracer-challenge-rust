use crate::geometry::{Point, Vector};

pub struct Ray<'a> {
    pub origin: &'a Point,
    pub direction: &'a Vector,
}

impl<'a> Ray<'a> {
    pub fn new(origin: &'a Point, direction: &'a Vector) -> Ray<'a> {
        if !direction.is_unit() {
            panic!("Only allow unit length vectors as ray direction")
        }
        Ray { origin, direction }
    }

    pub fn position(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }

    pub fn scalar_projection_of(&self, v: &Vector) -> f64 {
        self.direction.dot(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_ray_from_normalized_direction_vector() {
        let origin = Point(2., 3., 4.);
        let direction = Vector(1., 0., 0.);
        Ray::new(&origin, &direction);
    }

    #[test]
    #[should_panic]
    fn disallow_creating_rays_with_non_unit_direction_vector() {
        let origin = Point(2., 3., 4.);
        let direction = Vector(1., 1., 0.);
        Ray::new(&origin, &direction);
    }

    #[test]
    fn compute_point_from_distance() {
        let origin = Point(2., 3., 4.);
        let direction = Vector(1., 0., 0.);
        let ray = Ray::new(&origin, &direction);

        assert_eq!(Point(2., 3., 4.), ray.position(0.));
        assert_eq!(Point(3., 3., 4.), ray.position(1.));
        assert_eq!(Point(1., 3., 4.), ray.position(-1.));
        assert_eq!(Point(4.5, 3., 4.), ray.position(2.5));
    }

    #[test]
    fn compute_vector_projection() {
        let origin = Point(2., 3., 4.);
        let direction = Vector(1., 0., 0.);
        let ray = Ray::new(&origin, &direction);

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
