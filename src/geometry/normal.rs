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
