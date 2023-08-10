use crate::{
    drawing::Color,
    geometry::{Normal, UnitVector, Vector},
};

pub struct Material {
    color: Color,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: i32,
}

impl Material {
    pub const fn default() -> Material {
        Material {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200,
        }
    }

    pub fn lighting(
        &self,
        light_direction: &UnitVector,
        eye_direction: &Vector,
        normal: &Normal,
    ) -> Color {
        let light_intensity = Color::WHITE;

        let effective_color = &self.color * &light_intensity;
        let diffuse = self.diffuse(light_direction, normal);
        let specular = self.specular(light_direction, eye_direction, normal);

        &effective_color * (self.ambient + diffuse) + light_intensity * specular
    }

    fn diffuse(&self, light_direction: &UnitVector, normal: &Normal) -> f64 {
        let mut diffuse = 0.;

        let light_dot_normal = normal.dot(light_direction);
        if light_dot_normal >= 0. {
            diffuse = self.diffuse * light_dot_normal;
        }
        diffuse
    }

    fn specular(
        &self,
        light_direction: &UnitVector,
        eye_direction: &Vector,
        normal: &Normal,
    ) -> f64 {
        let mut specular = 0.;

        let reflection = normal.reflect(&(-light_direction));
        let reflect_dot_eye = (eye_direction).dot(&reflection);
        if reflect_dot_eye > 0. {
            let factor = reflect_dot_eye.powi(self.shininess);
            specular = self.specular * factor;
        }
        specular
    }
}

#[cfg(test)]
mod tests {
    mod default_material_lighting {
        use crate::{
            drawing::Color,
            geometry::{Normal, Vector},
            raycaster::Material,
        };

        const MATERIAL: Material = Material::default();

        #[test]
        fn light_strictly_behind_observer() {
            let light_d = Vector(0., 0., -10.).normalize();
            let eye_d = Vector(0., 0., -1.);
            let normal = Normal::new(0., 0., -1.);

            assert_eq!(
                0.9,
                MATERIAL.specular(&light_d, &eye_d, &normal),
                "Specular"
            );

            let result = MATERIAL.lighting(&light_d, &eye_d, &normal);
            assert_eq!(result, Color::new(1.9, 1.9, 1.9))
        }

        #[test]
        fn light_behind_observer_with_observer_offset() {
            let light_d = Vector(0., 0., -10.).normalize();
            let eye_d = Vector(0., 1., -1.).normalize();
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light_d, &eye_d, &normal);
            assert_eq!(result, Color::new(1.0, 1.0, 1.0))
        }

        #[test]
        fn observer_opposite_surface_with_light_offset() {
            let light_d = Vector(0., 10., -10.).normalize();
            let eye_d = Vector(0., 0., -1.);
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light_d, &eye_d, &normal);
            assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364))
        }

        #[test]
        fn observer_in_path_of_reflection_vector() {
            let light_d = Vector(0., 10., -10.).normalize();
            let eye_d = Vector(0., -1., -1.).normalize();
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light_d, &eye_d, &normal);
            assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364))
        }

        #[test]
        fn light_behind_surface() {
            let light_d = Vector(0., 0., 10.).normalize();
            let eye_d = Vector(0., 0., -1.);
            let normal = Normal::new(0., 0., -1.);

            let result = MATERIAL.lighting(&light_d, &eye_d, &normal);
            assert_eq!(result, Color::new(0.1, 0.1, 0.1))
        }
    }
}
