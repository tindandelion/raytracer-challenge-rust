use crate::{
    geometry::{Normal, Point},
    raycaster::Ray,
};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub const fn unit() -> Sphere {
        Self::new(Point::ZERO, 1.0)
    }

    pub const fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn normal_at(&self, pt: &Point) -> Normal {
        Normal::from(&(pt - &self.center))
    }

    pub fn intersect_with(&self, r: &Ray) -> Vec<f64> {
        let sphere_to_ray = r.origin - &self.center;

        let b = 2. * r.scalar_projection_of(&sphere_to_ray);
        let c = sphere_to_ray.magnitude_squared() - self.radius;

        solve_quadratic_equation(1., b, c)
            .map(|(x1, x2)| vec![x1, x2])
            .unwrap_or(vec![])
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
    use crate::geometry::Vector;

    use super::Sphere;

    static SPHERE: Sphere = Sphere::unit();
    static Z_AXIS: Vector = Vector(0., 0., 1.);

    mod sphere_normals {
        use super::super::Normal;
        use super::SPHERE;

        use crate::geometry::Point;

        #[test]
        fn normal_towards_x_axis() {
            let n = SPHERE.normal_at(&Point(1., 0., 0.));
            assert_eq!(n, Normal::new(1., 0., 0.));
        }

        #[test]
        fn normal_towards_y_axis() {
            let n = SPHERE.normal_at(&Point(0., 1., 0.));
            assert_eq!(n, Normal::new(0., 1., 0.));
        }

        #[test]
        fn normal_towards_z_axis() {
            let n = SPHERE.normal_at(&Point(0., 0., 1.));
            assert_eq!(n, Normal::new(0., 0., 1.));
        }

        #[test]
        fn normal_at_non_axial_point() {
            let sqrt_3 = (3.0 as f64).sqrt();
            let n = SPHERE.normal_at(&Point(sqrt_3 / 3., sqrt_3 / 3., sqrt_3 / 3.));
            assert_eq!(n, Normal::new(sqrt_3 / 3., sqrt_3 / 3., sqrt_3 / 3.))
        }
    }

    mod intersection {
        use super::{SPHERE, Z_AXIS};
        use crate::{geometry::Point, raycaster::Ray};

        #[test]
        fn ray_misses_sphere() {
            let xs = intersections_with_ray_from_origin(Point(0., 2., -5.));
            assert_eq!(xs, vec![]);
        }

        #[test]
        fn intersects_at_two_points() {
            let xs = intersections_with_ray_from_origin(Point(0., 0., -5.));
            assert_eq!(xs, vec![4.0, 6.0]);
        }

        #[test]
        fn intersects_at_tangent() {
            let xs = intersections_with_ray_from_origin(Point(0., 1., -5.));
            assert_eq!(xs, vec![5.0, 5.0]);
        }

        #[test]
        fn ray_originates_inside_sphere() {
            let xs = intersections_with_ray_from_origin(Point(0., 0., 0.));
            assert_eq!(xs, vec![-1., 1.]);
        }

        #[test]
        fn sphere_is_behind_ray() {
            let xs = intersections_with_ray_from_origin(Point(0., 0., 5.));
            assert_eq!(xs, vec![-6., -4.]);
        }

        fn intersections_with_ray_from_origin(origin: Point) -> Vec<f64> {
            let ray = Ray::new(&origin, &Z_AXIS);
            SPHERE.intersect_with(&ray)
        }
    }
}
