use crate::geometry::{Matrix, Point, Vector};

pub struct ViewTransform(Matrix);

impl ViewTransform {
    pub fn new(from: &Point, to: &Point, up: &Vector) -> ViewTransform {
        let view_z = (to - from).normalize();
        let up_norm = up.normalize();
        let view_x = up_norm.v().cross(view_z.v()).normalize();
        let view_y = view_z.v().cross(view_x.v()).normalize();

        let orientation = Matrix::from_vectors(&view_x, &view_y, &view_z);
        let transform = Matrix::translation(from.as_vector()) * orientation;
        ViewTransform(transform)
    }

    pub fn to(to: &Point) -> ViewTransform {
        Self::new(&Point::ZERO, to, &Vector(0., 1., 0.))
    }

    pub fn to_world(&self, view_point: &Point) -> Point {
        &self.0 * view_point
    }
}

#[cfg(test)]
mod tests {
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
