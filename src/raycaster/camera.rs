use crate::geometry::{Matrix, Point, Ray, Vector};

pub struct Camera {
    h_size: usize,
    v_size: usize,
    half_view: f64,
    aspect_ratio: f64,
    pixel_size: f64,
    transform: ViewTransform,
}

struct ViewTransform(Matrix);

impl ViewTransform {
    fn new(from: &Point, to: &Point, up: &Vector) -> ViewTransform {
        let view_z = (to - from).normalize();
        let up_norm = up.normalize();
        let view_x = up_norm.v().cross(view_z.v()).normalize();
        let view_y = view_z.v().cross(view_x.v()).normalize();

        let orientation = Matrix::from_vectors(&view_x, &view_y, &view_z);
        let transform = Matrix::translation(from.as_vector()) * orientation;
        ViewTransform(transform)
    }

    fn default() -> ViewTransform {
        Self::new(&Point::ZERO, &Point::new(0., 0., -1.), &Vector(0., 1., 0.))
    }

    fn to_world(&self, view_point: &Point) -> Point {
        &self.0 * view_point
    }
}

impl Camera {
    pub fn new(h_size: usize, v_size: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.).tan();
        let aspect_ratio = (v_size as f64) / (h_size as f64);
        let pixel_size = if aspect_ratio < 1. {
            2. * half_view / (h_size as f64)
        } else {
            2. * half_view / (v_size as f64)
        };
        let transform = ViewTransform::default();

        Camera {
            h_size,
            v_size,
            half_view,
            pixel_size,
            aspect_ratio,
            transform,
        }
    }

    pub fn h_size(&self) -> usize {
        self.h_size
    }

    pub fn v_size(&self) -> usize {
        self.v_size
    }

    pub fn with_transform(mut self, from: &Point, to: &Point, up: &Vector) -> Self {
        self.transform = ViewTransform::new(from, to, up);
        self
    }

    pub fn scan_space(&self, mut f: impl FnMut(&Ray, usize, usize) -> ()) {
        for y in 0..self.v_size {
            for x in 0..self.h_size {
                self.cast_ray_at(x, y, |r| f(&r, x, y));
            }
        }
    }

    pub fn cast_ray_at(&self, px: usize, py: usize, mut f: impl FnMut(&Ray) -> ()) {
        let origin = self.transform.to_world(&Point::ZERO);
        let pixel = self.transform.to_world(&self.view_pixel_at(px, py));
        f(&Ray::between(&origin, &pixel))
    }

    fn view_pixel_at(&self, px: usize, py: usize) -> Point {
        let x_offset = (px as f64 + 0.5) * self.pixel_size;
        let y_offset = (py as f64 + 0.5) * self.pixel_size;
        let world_x = -self.half_view + x_offset;
        let world_y = self.half_view * self.aspect_ratio - y_offset;
        Point::new(world_x, world_y, 1.)
    }
}

#[cfg(test)]
mod camera_tests {
    use std::f64::consts::PI;

    use crate::geometry::{Point, Vector};

    use super::Camera;

    #[test]
    fn pixel_size_of_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.);
        assert_approx_eq(c.pixel_size, 0.01);
    }

    #[test]
    fn pixel_size_of_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.);
        assert_approx_eq(c.pixel_size, 0.01);
    }

    #[test]
    fn direction_to_the_canvas_center() {
        let c = Camera::new(201, 101, PI / 2.);
        c.cast_ray_at(100, 50, |r| {
            assert_eq!(r.direction.v(), &Vector(0., 0., -1.))
        });
    }

    #[test]
    fn direction_to_canvas_corner() {
        let c = Camera::new(201, 101, PI / 2.);
        c.cast_ray_at(0, 0, |r| {
            assert_eq!(r.direction.v(), &Vector(0.665186, 0.332593, -0.668512))
        });
    }

    #[test]
    fn direction_to_canvas_corner_with_positive_z_direction() {
        let c = Camera::new(201, 101, PI / 2.).with_transform(
            &Point::ZERO,
            &Point::new(0., 0., 1.),
            &Vector(0., 1., 0.),
        );

        c.cast_ray_at(0, 0, |r| {
            assert_eq!(r.direction.v(), &Vector(-0.665186, 0.332593, 0.668512))
        });
    }

    fn assert_approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 0.0001, "{:?} != {:?}", left, right);
    }
}

#[cfg(test)]
mod view_transform_tests {
    use std::f64::consts::SQRT_2;

    use crate::geometry::{Point, UnitVector, Vector};

    use super::ViewTransform;

    #[test]
    fn choose_view_direction_in_xz_plane() {
        let view_up = UnitVector::Y.v();
        let look_at = Point::new(1., 0., -1.);
        let transform = ViewTransform::new(&Point::ZERO, &look_at, &view_up);

        let view_point = Point::new(0., 0., SQRT_2);
        let world_point = transform.to_world(&view_point);
        assert_eq!(world_point, look_at);
    }

    #[test]
    fn specify_custom_up_direction() {
        let look_at = Point::new(0., 0., 1.);
        let up = Vector(1., 1., 1.);
        let transform = ViewTransform::new(&Point::ZERO, &look_at, &up);

        let view_point = Point::new(1., 1., 5.);
        let world_point = transform.to_world(&view_point);
        assert_eq!(world_point, Point::new(SQRT_2, 0., 5.))
    }

    #[test]
    fn specify_custom_from_point() {
        let view_up = UnitVector::Y.v();
        let look_at = Point::ZERO;
        let look_from = Point::new(2., 0., -2.);
        let transform = ViewTransform::new(&look_from, &look_at, &view_up);

        let view_point = Point::new(0., 0., SQRT_2);
        let world_point = transform.to_world(&view_point);
        assert_eq!(world_point, Point::new(1., 0., -1.))
    }
}
