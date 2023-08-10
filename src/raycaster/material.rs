use crate::{
    drawing::Color,
    geometry::{Normal, UnitVector, Vector},
};

pub struct Material;

impl Material {
    pub fn default() -> Material {
        Material {}
    }

    pub fn lighting(
        &self,
        light_direction: &UnitVector,
        eye_direction: &Vector,
        normal: &Normal,
    ) -> Color {
        let reflection = normal.reflect(&light_direction);
        let cos_alpha = eye_direction.dot(&reflection);
        let luminosity = cos_alpha.max(0.);
        Color::new(luminosity, 0., 0.) + Color::new(0.1, 0., 0.)
    }
}
