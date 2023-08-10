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

    pub fn dot(&self, v: &Vector) -> f64 {
        self.direction.dot(v)
    }

    pub fn reflect(&self, incoming: &Vector) -> Vector {
        -incoming + &self.direction * 2. * incoming.dot(&self.direction)
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

        assert_eq!(n.direction, Vector(SQRT_2 / 2., SQRT_2 / 2., 0.));
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
