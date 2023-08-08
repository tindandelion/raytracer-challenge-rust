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
}
