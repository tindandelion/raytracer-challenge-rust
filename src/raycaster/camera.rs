use crate::geometry::Point;

use super::{view_transform::ViewTransform, Ray};

pub struct Camera {
    half_view: f64,
    aspect_ratio: f64,
    pixel_size: f64,
    transform: ViewTransform,
}

impl Camera {
    const CAMERA_POSITION: Point = Point::ZERO;

    pub fn new(h_size: usize, v_size: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.).tan();
        let aspect_ratio = (v_size as f64) / (h_size as f64);
        let pixel_size = if aspect_ratio < 1. {
            2. * half_view / (h_size as f64)
        } else {
            2. * half_view / (v_size as f64)
        };
        let transform = ViewTransform::to(&Point::new(0., 0., -1.));

        Camera {
            half_view,
            pixel_size,
            aspect_ratio,
            transform,
        }
    }

    pub fn set_transform(&mut self, transform: ViewTransform) {
        self.transform = transform;
    }

    pub fn cast_ray_at(&self, px: usize, py: usize, mut f: impl FnMut(&Ray) -> ()) {
        let origin = self.transform.to_world(&Self::CAMERA_POSITION);
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
mod tests {
    use std::f64::consts::PI;

    use crate::{
        geometry::{Point, Vector},
        raycaster::view_transform::ViewTransform,
    };

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
        let mut c = Camera::new(201, 101, PI / 2.);
        c.set_transform(ViewTransform::to(&Point::new(0., 0., 1.)));
        c.cast_ray_at(0, 0, |r| {
            assert_eq!(r.direction.v(), &Vector(-0.665186, 0.332593, 0.668512))
        });
    }

    fn assert_approx_eq(left: f64, right: f64) {
        assert!((left - right).abs() < 0.0001, "{:?} != {:?}", left, right);
    }
}
