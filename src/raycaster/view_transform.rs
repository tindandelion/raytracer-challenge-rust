use crate::geometry::{Point, Vector};

struct ViewTransform {
    matrix: [f64; 16],
}

impl ViewTransform {
    pub fn identity() -> ViewTransform {
        #[rustfmt::skip]    
        let matrix = [
            1., 0., 0., 0., 
            0., 1., 0., 0., 
            0., 0., 1., 0., 
            0., 0., 0., 1.
        ];

        ViewTransform {
            matrix
        }
    }

    pub fn to(point: &Point) -> ViewTransform {
        let origin = Point::ZERO;
        let up = Vector(0., 1., 0.);
        Self::identity()
    }

    pub fn to_view(&self, world_point: &Point) -> Point {
        let v = world_point.as_vector();
        Point::new(
            v.0 * self.el(0, 0) + v.1 * self.el(0, 1) + v.2 * self.el(0, 2),
            v.0 * self.el(1, 0) + v.1 * self.el(1, 1) + v.2 * self.el(1, 2),
            v.0 * self.el(2, 0) + v.1 * self.el(2, 1) + v.2 * self.el(2, 2),
        )
    }

    pub fn to_world(&self, view_point: &Point) -> Point {
        view_point.clone()
    }

    fn el(&self, row: usize, col: usize) -> f64 {
        self.matrix[row * 4 + col]
    }
}

#[cfg(test)]
mod tests {
    use crate::geometry::{Point, Vector};

    use super::ViewTransform;

    #[test] 
    fn construct_new_basis() {
        let origin = Point::ZERO;
        let view_direction = Point::new(0., 0., -1.);

        let view_y = Vector(0., 1., 0.).normalize();
        let view_z = (view_direction - origin).normalize();
        let view_x = view_y.v().cross(view_z.v()).normalize();

        assert_eq!(view_x.v(), &Vector(-1., 0., 0.))
    }


    #[test]
    fn identity_transform() {
        let world_point = Point::new(1., 2., 3.);
        let transform = ViewTransform::identity();

        let view_point = transform.to_view(&world_point);
        let restored_world_point = transform.to_world(&view_point);

        assert_eq!(view_point, world_point);
        assert_eq!(restored_world_point, world_point);
    }

    #[ignore]
    #[test]
    fn look_to_opposite_direction() {
        let view_direction = Point::new(0., 0., -1.);
        let transform = ViewTransform::to(&view_direction);

        let world_point = Point::new(1., 2., 3.);
        let view_point = transform.to_view(&world_point);
        assert_eq!(view_point, Point::new(-1., 2., -3.))
    }
}
