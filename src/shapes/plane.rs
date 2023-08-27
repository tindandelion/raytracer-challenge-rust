use crate::geometry::{self, Normal, Point, UnitVector};

use super::{Material, Shape};

pub struct Plane {
    material: Material,
}

impl Plane {
    const EPSILON: f64 = 1.0e-4;

    pub const fn new() -> Plane {
        Plane {
            material: Material::default(),
        }
    }

    pub fn with_material(mut self, m: Material) -> Self {
        self.material = m;
        self
    }
}

impl Shape for Plane {
    fn material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, _pt: &Point) -> Normal {
        Normal::from(UnitVector::Y.v())
    }

    fn intersect_with(&self, ray: &geometry::Ray) -> Vec<f64> {
        let direction_y = ray.direction.v().1;
        if direction_y.abs() < Plane::EPSILON {
            vec![]
        } else {
            vec![-ray.origin.y() / direction_y]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::Ray;

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
        let ray_origin = Point::new(0., 10., 0.);
        let ray = Ray::new(&ray_origin, UnitVector::Z);

        let intersections = PLANE.intersect_with(&ray);
        assert!(intersections.is_empty())
    }

    #[test]
    pub fn intersect_with_coplanar_ray() {
        let ray_origin = Point::new(0., 0., 0.);
        let ray = Ray::new(&ray_origin, UnitVector::Z);

        let intersections = PLANE.intersect_with(&ray);
        assert!(intersections.is_empty())
    }

    #[test]
    pub fn intersect_with_ray_from_above() {
        let ray_origin = Point::new(0., 1., 0.);
        let ray = Ray::new(&ray_origin, UnitVector::Y.flip());

        let intersections = PLANE.intersect_with(&ray);
        assert_eq!(intersections, vec![1.])
    }

    #[test]
    pub fn intersect_with_ray_from_below() {
        let ray_origin = Point::new(0., -1., 0.);
        let ray = Ray::new(&ray_origin, UnitVector::Y);

        let intersections = PLANE.intersect_with(&ray);
        assert_eq!(intersections, vec![1.])
    }
}
