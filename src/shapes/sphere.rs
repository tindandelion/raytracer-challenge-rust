use crate::geometry::{Normal, Point, Ray, Vector};

use super::{Material, Shape, Transform};

pub struct Sphere {
    radius: f64,
    material: Material,
    transform: Transform,
}

impl Shape for Sphere {
    fn material(&self) -> &Material {
        &self.material
    }

    fn normal_at(&self, pt: &Point) -> Normal {
        let local_point = self.transform.inverse().apply(pt);
        self.local_normal_at(&local_point)
    }

    fn intersect_with(&self, r: &Ray) -> Vec<f64> {
        let local_ray = self.transform.inverse().apply(r);
        self.local_intersect_with(&local_ray)
    }
}

impl Sphere {
    pub const fn new(radius: f64) -> Sphere {
        Sphere {
            radius,
            material: Material::default(),
            transform: Transform::IDENTITY,
        }
    }

    pub fn with_material(mut self, material: Material) -> Sphere {
        self.material = material;
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Sphere {
        self.transform = transform;
        self
    }

    fn local_intersect_with(&self, r: &Ray) -> Vec<f64> {
        let sphere_to_ray = &r.origin;
        let b = 2. * r.scalar_projection_of(&sphere_to_ray);
        let c = sphere_to_ray.magnitude_squared() - self.radius * self.radius;

        solve_quadratic_equation(1., b, c)
            .map(|(x1, x2)| vec![x1, x2])
            .unwrap_or(vec![])
    }

    fn local_normal_at(&self, pt: &Point) -> Normal {
        Normal::from(pt as &Vector)
    }
}

fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let d = b * b - 4. * a * c;
    if d < 0. {
        None
    } else {
        let sqrt_d = d.sqrt();
        let x1 = (-b - sqrt_d) / (2. * a);
        let x2 = (-b + sqrt_d) / (2. * a);
        Some((x1, x2))
    }
}

#[cfg(test)]
mod tests {

    use super::Sphere;

    static SPHERE: Sphere = Sphere::new(1.0);

    mod sphere_normals {
        use super::super::Normal;
        use super::SPHERE;

        use crate::geometry::Point;
        use crate::shapes::Shape;

        #[test]
        fn normal_towards_x_axis() {
            let n = SPHERE.normal_at(&Point::new(1., 0., 0.));
            assert_eq!(n, Normal::new(1., 0., 0.));
        }

        #[test]
        fn normal_towards_y_axis() {
            let n = SPHERE.normal_at(&Point::new(0., 1., 0.));
            assert_eq!(n, Normal::new(0., 1., 0.));
        }

        #[test]
        fn normal_towards_z_axis() {
            let n = SPHERE.normal_at(&Point::new(0., 0., 1.));
            assert_eq!(n, Normal::new(0., 0., 1.));
        }

        #[test]
        fn normal_at_non_axial_point() {
            let sqrt_3 = (3.0 as f64).sqrt();
            let n = SPHERE.normal_at(&Point::new(sqrt_3 / 3., sqrt_3 / 3., sqrt_3 / 3.));
            assert_eq!(n, Normal::new(sqrt_3 / 3., sqrt_3 / 3., sqrt_3 / 3.))
        }
    }

    mod basic_intersection {
        use super::SPHERE;
        use crate::{
            geometry::{Point, Ray, UnitVector},
            shapes::Shape,
        };

        #[test]
        fn ray_misses_sphere() {
            let xs = intersections_with_ray_from_origin(Point::new(0., 2., -5.));
            assert_eq!(xs, vec![]);
        }

        #[test]
        fn intersects_at_two_points() {
            let xs = intersections_with_ray_from_origin(Point::new(0., 0., -5.));
            assert_eq!(xs, vec![4.0, 6.0]);
        }

        #[test]
        fn intersects_at_tangent() {
            let xs = intersections_with_ray_from_origin(Point::new(0., 1., -5.));
            assert_eq!(xs, vec![5.0, 5.0]);
        }

        #[test]
        fn ray_originates_inside_sphere() {
            let xs = intersections_with_ray_from_origin(Point::new(0., 0., 0.));
            assert_eq!(xs, vec![-1., 1.]);
        }

        #[test]
        fn sphere_is_behind_ray() {
            let xs = intersections_with_ray_from_origin(Point::new(0., 0., 5.));
            assert_eq!(xs, vec![-6., -4.]);
        }

        fn intersections_with_ray_from_origin(origin: Point) -> Vec<f64> {
            let ray = Ray::new(origin, UnitVector::Z);
            SPHERE.intersect_with(&ray)
        }
    }

    mod transformed_sphere {
        use crate::{
            geometry::{Normal, Point, Ray, UnitVector},
            shapes::{Shape, Sphere, Transform},
        };

        #[test]
        fn intersect_translated_sphere_with_ray() {
            let sphere = Sphere::new(1.0).with_transform(Transform::translate(0., 0., 5.));

            let ray = Ray::new(Point::new(0., 0., -5.), UnitVector::Z);
            let xs = sphere.intersect_with(&ray);
            assert_eq!(xs, vec![9., 11.]);
        }

        #[test]
        fn normal_of_translated_sphere() {
            let sphere = Sphere::new(1.0).with_transform(Transform::translate(0., 1., 0.));

            let normal = sphere.normal_at(&Point::new(1., 1., 0.));
            assert_eq!(normal, Normal::new(1., 0., 0.))
        }
    }
}
