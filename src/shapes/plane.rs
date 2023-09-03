use crate::geometry::{Normal, Point, Ray, UnitVector};

use super::{Material, Shape, Transform};

pub struct Plane {
    material: Material,
    transform: Transform,
}

impl Plane {
    const EPSILON: f64 = 1.0e-4;

    pub const fn new() -> Plane {
        Plane {
            material: Material::default(),
            transform: Transform::IDENTITY,
        }
    }

    pub fn with_material(mut self, m: Material) -> Self {
        self.material = m;
        self
    }

    pub fn with_transform(mut self, t: Transform) -> Self {
        self.transform = t;
        self
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, _pt: &Point) -> Normal {
        let vector = self.transform.apply(&UnitVector::Y);
        Normal::from(&vector)
    }

    fn intersect_with(&self, ray: &Ray) -> Vec<f64> {
        let local_ray = self.transform.inverse().apply(ray);
        let direction_y = local_ray.direction.1;
        if direction_y.abs() < Plane::EPSILON {
            vec![]
        } else {
            vec![-local_ray.origin.y() / direction_y]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::{
        geometry::{Ray, Vector},
        shapes::Transform,
    };

    use super::*;

    const PLANE: Plane = Plane::new();

    #[test]
    pub fn normal_is_constant() {
        let y_axis_normal = Normal::new(0., 1., 0.);

        assert_eq!(PLANE.normal_at(&Point::ZERO), y_axis_normal);
        assert_eq!(PLANE.normal_at(&Point::new(10., 0., -10.)), y_axis_normal);
        assert_eq!(PLANE.normal_at(&Point::new(-5., 0., 150.)), y_axis_normal);
    }

    #[test]
    pub fn intersect_with_parallel_ray() {
        let ray = Ray::new(Point::new(0., 10., 0.), UnitVector::Z);

        let intersections = PLANE.intersect_with(&ray);
        assert!(intersections.is_empty())
    }

    #[test]
    pub fn intersect_with_coplanar_ray() {
        let ray = Ray::new(Point::new(0., 0., 0.), UnitVector::Z);

        let intersections = PLANE.intersect_with(&ray);
        assert!(intersections.is_empty())
    }

    #[test]
    pub fn intersect_with_ray_from_above() {
        let ray = Ray::new(Point::new(0., 1., 0.), UnitVector::Y.flip());

        let intersections = PLANE.intersect_with(&ray);
        assert_eq!(intersections, vec![1.])
    }

    #[test]
    pub fn intersect_with_ray_from_below() {
        let ray = Ray::new(Point::new(0., -1., 0.), UnitVector::Y);

        let intersections = PLANE.intersect_with(&ray);
        assert_eq!(intersections, vec![1.])
    }

    #[test]
    pub fn apply_transform_to_normal() {
        let transform = Transform::rotate_x(PI / 4.);
        let plane = Plane::new().with_transform(transform);

        let normal = plane.normal_at(&Point::ZERO);
        assert_eq!(normal, Normal::new(0., SQRT_2 / 2., SQRT_2 / 2.))
    }

    #[test]
    pub fn apply_transform_to_ray() {
        let transform = Transform::translate(&Vector(0., -1., 0.));
        let plane = Plane::new().with_transform(transform);

        let ray = Ray::new(Point::new(0., 1., 0.), UnitVector::Y.flip());
        let intersections = plane.intersect_with(&ray);
        assert_eq!(intersections, vec![2.])
    }
}
