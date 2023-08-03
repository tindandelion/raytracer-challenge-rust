use overload::overload;
use std::ops;

use crate::geometry::Point;

pub struct Transform {
    matrix: [f64; 16],
}

overload!((t: ?Transform) * (p: ?Point) -> Point { t.apply(&p) });

impl Transform {
    pub fn rotate_z(angle_z: f64) -> Transform {
        #[rustfmt::skip]    
        let matrix = [
            angle_z.cos(), -angle_z.sin(), 0., 0., 
            angle_z.sin(),  angle_z.cos(), 0., 0., 
            0.,             0.,            1., 0., 
            0.,             0.,            0., 1.
        ];
        Transform { matrix }
    }

    fn apply(&self, pt: &Point) -> Point {
        Point(
            pt.0 * self.el(0, 0) + pt.1 * self.el(0, 1) + pt.2 * self.el(0, 2),
            pt.0 * self.el(1, 0) + pt.1 * self.el(1, 1) + pt.2 * self.el(1, 2),
            pt.0 * self.el(2, 0) + pt.1 * self.el(2, 1) + pt.2 * self.el(2, 2),
        )
    }

    fn el(&self, row: usize, col: usize) -> f64 {
        self.matrix[row * 4 + col]
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::{PI, SQRT_2};

    use crate::geometry::Point;

    use super::Transform;

    #[test]
    fn rotate_around_z_axis() {
        let pt = Point(0., 1., 0.);
        let half_quarter = Transform::rotate_z(PI / 4.);
        let full_quarter = Transform::rotate_z(PI / 2.);

        assert_points_eq(half_quarter * &pt, Point(-SQRT_2 / 2., SQRT_2 / 2., 0.));
        assert_points_eq(full_quarter * &pt, Point(-1., 0., 0.));
    }

    fn assert_points_eq(p1: Point, p2: Point) {
        let allowed_delta = 0.0001;
        let point_delta = (&p1 - &p2).magnitude();
        assert!(point_delta < allowed_delta, "{:?} != {:?}", p1, p2);
    }
}
