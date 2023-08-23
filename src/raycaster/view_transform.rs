use crate::geometry::{Point, UnitVector, Vector};

struct OrthogonalMatrix([f64; 16]);

pub struct ViewTransform {
    view_basis: OrthogonalMatrix,
}

impl OrthogonalMatrix {
    fn from_vectors(x: &UnitVector, y: &UnitVector, z: &UnitVector) -> OrthogonalMatrix {
        let x_v = x.v();
        let y_v = y.v();
        let z_v = z.v();
        #[rustfmt::skip]
        let matrix = [
            x_v.0, y_v.0, z_v.0, 0., 
            x_v.1, y_v.1, z_v.1, 0., 
            x_v.2, y_v.2, z_v.2, 0., 
            0.,    0.,    0.,    1.,
        ];
        OrthogonalMatrix(matrix)
    }

    fn mul(&self, point: &Point) -> Point {
        let v = point.as_vector();
        Point::new(
            v.0 * self.el(0, 0) + v.1 * self.el(0, 1) + v.2 * self.el(0, 2),
            v.0 * self.el(1, 0) + v.1 * self.el(1, 1) + v.2 * self.el(1, 2),
            v.0 * self.el(2, 0) + v.1 * self.el(2, 1) + v.2 * self.el(2, 2),
        )
    }

    fn inverse(&self) -> OrthogonalMatrix {
        self.transpose()
    }

    fn transpose(&self) -> OrthogonalMatrix {
        #[rustfmt::skip]
        let transposed = [
            self.el(0, 0), self.el(1, 0), self.el(2, 0), self.el(3, 0),
            self.el(0, 1), self.el(1, 1), self.el(2, 1), self.el(3, 1),
            self.el(0, 2), self.el(1, 2), self.el(2, 2), self.el(3, 2),
            self.el(0, 3), self.el(1, 3), self.el(2, 3), self.el(3, 3),
        ];
        OrthogonalMatrix(transposed)
    }

    fn el(&self, row: usize, col: usize) -> f64 {
        self.0[row * 4 + col]
    }
}

impl ViewTransform {
    pub fn to(view_direction: &Point) -> ViewTransform {
        Self::to_up(view_direction, &Vector(0., 1., 0.))
    }

    pub fn to_up(to: &Point, up: &Vector) -> ViewTransform {
        let from = Point::ZERO;
        let view_z = (to - from).normalize();

        let up_norm = up.normalize();
        let view_x = up_norm.v().cross(view_z.v()).normalize();
        let view_y = view_z.v().cross(view_x.v()).normalize();
        

        let view_basis = OrthogonalMatrix::from_vectors(&view_x, &view_y, &view_z);
        ViewTransform { view_basis }
    }

    pub fn to_world(&self, view_point: &Point) -> Point {
        self.view_basis.mul(view_point)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::geometry::{Point, Vector};

    use super::ViewTransform;

    #[test]
    fn explore_cross_product() {
        let x = Vector(1., 0., 0.);
        let y = Vector(0., 1., 0.);
        let z = Vector(0., 0., 1.);

        assert_eq!(x, y.cross(&z));
        assert_eq!(y, z.cross(&x));
    }

    #[test]
    fn look_in_positive_z_direction() {
        let view_direction = Point::new(0., 0., 1.);
        let transform = ViewTransform::to(&view_direction);

        let view_point = Point::new(1., 0., 1.);
        let world_point = transform.to_world(&view_point);
        assert_eq!(world_point, view_point);
    }

    #[test]
    fn look_in_negative_z_direction() {
        let view_direction = Point::new(0., 0., -1.);
        let transform = ViewTransform::to(&view_direction);

        let view_point = Point::new(1., 0., 1.);
        let world_point = transform.to_world(&view_point);
        assert_eq!(world_point, Point::new(-1., 0., -1.));
    }

    #[test]
    fn choose_view_direction_in_xz_plane() {
        let view_direction = Point::new(1., 0., -1.);
        let transform = ViewTransform::to(&view_direction);
        
        let view_point = Point::new(0., 0., SQRT_2);
        let world_point = transform.to_world(&view_point);
        assert_eq!(world_point, view_direction);
    }

    #[test]
    fn specify_custom_up_direction() {
        let view_direction = Point::new(0., 0., 1.);
        let up = Vector(1., 1., 1.);
        let transform = ViewTransform::to_up(&view_direction, &up);
        

        let view_point = Point::new(1., 1., 5.);
        let world_point = transform.to_world(&view_point);
        assert_eq!(world_point, Point::new(SQRT_2, 0., 5.))
    }
}
