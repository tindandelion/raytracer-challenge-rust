use crate::{
    drawing::Color,
    geometry::{Normal, Point, UnitVector},
};

use super::PointLight;

pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: i32,
}

impl Material {
    pub const fn default_with_color(color: Color) -> Material {
        Material {
            color,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200,
        }
    }

    pub fn lighting(
        &self,
        light: &PointLight,
        position: &Point,
        eye_direction: &UnitVector,
        normal: &Normal,
    ) -> Color {
        let light_direction = light.direction_from(position);
        let diffuse_factor = self.diffuse(&light_direction, normal);
        let specular_factor = self.specular(&light_direction, eye_direction, normal);

        let effective_color = &light.intensity * &self.color * (self.ambient + diffuse_factor);
        &effective_color + &light.intensity * specular_factor
    }

    fn diffuse(&self, light_direction: &UnitVector, normal: &Normal) -> f64 {
        let light_dot_normal = normal.dot(light_direction);
        if light_dot_normal < 0. {
            return 0.;
        }
        self.diffuse * light_dot_normal
    }

    fn specular(
        &self,
        light_direction: &UnitVector,
        eye_direction: &UnitVector,
        normal: &Normal,
    ) -> f64 {
        let reflection = normal.reflect(light_direction);
        let reflect_dot_eye = (eye_direction).dot(&reflection);
        if (reflect_dot_eye) <= 0. {
            return 0.;
        }
        let factor = reflect_dot_eye.powi(self.shininess);
        self.specular * factor
    }
}

#[cfg(test)]
mod tests {
    mod default_material_lighting {
        use crate::{
            drawing::Color,
            geometry::{Normal, Point, Vector},
            raycaster::{Material, PointLight},
        };

        const MATERIAL: Material = Material::default_with_color(Color::WHITE);
        const POSITION: Point = Point::ZERO;

        #[test]
        fn light_strictly_behind_observer() {
            let light = PointLight::new(Color::WHITE, Point::new(0., 0., -10.));
            let eye_d = Vector(0., 0., -1.);
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light, &POSITION, &eye_d, &normal);
            assert_eq!(result, Color::new(1.9, 1.9, 1.9))
        }

        #[test]
        fn light_behind_observer_with_observer_offset() {
            let light = PointLight::new(Color::WHITE, Point::new(0., 0., -10.));
            let eye_d = Vector(0., 1., -1.).normalize();
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light, &POSITION, &eye_d, &normal);
            assert_eq!(result, Color::new(1.0, 1.0, 1.0))
        }

        #[test]
        fn observer_opposite_surface_with_light_offset() {
            let light = PointLight::new(Color::WHITE, Point::new(0., 10., -10.));
            let eye_d = Vector(0., 0., -1.);
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light, &POSITION, &eye_d, &normal);
            assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364))
        }

        #[test]
        fn observer_in_path_of_reflection_vector() {
            let light = PointLight::new(Color::WHITE, Point::new(0., 10., -10.));
            let eye_d = Vector(0., -1., -1.).normalize();
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light, &POSITION, &eye_d, &normal);
            assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364))
        }

        #[test]
        fn light_behind_surface() {
            let light = PointLight::new(Color::WHITE, Point::new(0., 0., 10.));
            let eye_d = Vector(0., 0., -1.);
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light, &POSITION, &eye_d, &normal);
            assert_eq!(result, Color::new(0.1, 0.1, 0.1))
        }

        #[test]
        fn use_light_intensity_to_calculate_result() {
            let light = PointLight::new(Color::new(1., 0., 0.), Point::new(0., 0., -10.));
            let eye_d = Vector(0., 1., -1.).normalize();
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light, &POSITION, &eye_d, &normal);
            assert_eq!(result, Color::new(1., 0., 0.))
        }

        #[test]
        fn use_material_color_to_calculate_result() {
            let material = Material::default_with_color(Color::new(1., 0., 0.));
            let light = PointLight::new(Color::WHITE, Point::new(0., 0., -10.));
            let eye_d = Vector(0., 1., -1.).normalize();
            let normal = Normal::new(0., 0., -1.);

            let result = material.lighting(&light, &POSITION, &eye_d, &normal);
            assert_eq!(result, Color::new(1., 0., 0.))
        }
    }
}
