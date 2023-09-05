use std::ops::Deref;

use super::{Point, UnitVector, Vector};

#[derive(PartialEq, Debug)]
pub struct Normal(UnitVector);

impl Normal {
    pub fn dot(&self, v: &Vector) -> f64 {
        self.0.dot(v)
    }

    pub fn reflect(&self, incoming: &Vector) -> Vector {
        -incoming + self.0.deref() * 2. * incoming.dot(&self.0)
    }

    pub fn flip(&self) -> Normal {
        Normal(self.0.flip())
    }

    pub fn over_point(&self, point: &Point) -> Point {
        point + self.0.deref() * 1e-8
    }
}

impl From<&Vector> for Normal {
    fn from(value: &Vector) -> Self {
        Normal(value.normalize())
    }
}

#[cfg(test)]
mod tests {
    use super::Normal;
    use crate::geometry::Vector;
    use std::f64::consts::SQRT_2;

    impl Normal {
        pub fn new(x: f64, y: f64, z: f64) -> Normal {
            Self::from(&Vector(x, y, z))
        }
    }

    #[test]
    fn create_normal_from_non_unit_vector() {
        let v = Vector(1., 1., 0.);
        let n = Normal::from(&v);

        assert_eq!(*n.0, Vector(SQRT_2 / 2., SQRT_2 / 2., 0.));
    }

    #[test]
    fn reflect_vector_at_45_degrees_from_horizontal_surface() {
        let v = Vector(1., 1., 0.);
        let n = Normal::new(0., 1., 0.);

        let r = n.reflect(&v);
        assert_eq!(r, Vector(-1., 1., 0.));
    }

    #[test]
    fn reflect_off_slanted_surface() {
        let v = Vector(0., 1., 0.);
        let n = Normal::new(SQRT_2 / 2., SQRT_2 / 2., 0.);

        let r = n.reflect(&v);
        assert_eq!(r, Vector(1., 0., 0.))
    }
}
