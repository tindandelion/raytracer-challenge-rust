use crate::geometry::{Point, Vector};

pub struct Ray<'a> {
    origin: &'a Point,
    direction: &'a Vector,
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
}
