use crate::{
    drawing::Color,
    geometry::{Point, UnitVector},
};

pub struct PointLight {
    pub intensity: Color,
    position: Point,
}

impl PointLight {
    pub const fn new(intensity: Color, position: Point) -> PointLight {
        PointLight {
            intensity,
            position,
        }
    }

    pub fn direction_from(&self, pt: &Point) -> UnitVector {
        (&self.position - pt).normalize()
    }

    pub fn distance_from(&self, point: &Point) -> f64 {
        (&self.position - point).magnitude()
    }
}
