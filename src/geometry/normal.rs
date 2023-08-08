use super::Vector;

#[derive(PartialEq, Debug)]
pub struct Normal {
    direction: Vector,
}

impl Normal {
    pub fn from(v: &Vector) -> Normal {
        Normal {
            direction: v.normalize(),
        }
    }

    pub fn new(x: f64, y: f64, z: f64) -> Normal {
        Self::from(&Vector(x, y, z))
    }

    pub fn reflect(&self, incoming: &Vector) -> Vector {
        incoming - &self.direction * 2. * incoming.dot(&self.direction)
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::SQRT_2;

    use crate::geometry::Vector;

    use super::Normal;

    #[test]
    fn create_normal_from_non_unit_vector() {
        let v = Vector(1., 1., 0.);
        let n = Normal::from(&v);

        assert_eq!(n.direction, Vector(SQRT_2 / 2., SQRT_2 / 2., 0.));
    }

    #[test]
    fn reflect_vector_at_45_degrees() {
        let v = Vector(1., -1., 0.);
        let n = Normal::new(0., 1., 0.);

        let r = n.reflect(&v);
        assert_eq!(r, Vector(1., 1., 0.));
    }

    #[test]
    fn reflect_off_slanted_surface() {
        let v = Vector(0., -1., 0.);
        let n = Normal::new(SQRT_2 / 2., SQRT_2 / 2., 0.);

        let r = n.reflect(&v);
        assert_eq!(r, Vector(1., 0., 0.))
    }
}
