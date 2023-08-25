use crate::{
    drawing::Color,
    geometry::{Point, UnitVector},
    shapes::Sphere,
};

use super::{PointLight, Ray};

pub struct World {
    light: PointLight,
    shapes: Vec<Sphere>,
}

struct RayHit<'a> {
    point: Point,
    shape: &'a Sphere,
}

impl RayHit<'_> {
    fn lightning(&self, light: &PointLight, ray_direction: &UnitVector) -> Color {
        let normal = self.shape.normal_at(&self.point);
        let eye_direction = ray_direction.flip();
        self.shape
            .material()
            .lighting(light, &self.point, &eye_direction, &normal)
    }
}

impl World {
    pub fn new(light: PointLight) -> World {
        World {
            light,
            shapes: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: Sphere) {
        self.shapes.push(shape)
    }

    pub fn get_color(&self, ray: &Ray) -> Option<Color> {
        self.hit_with_ray(&ray)
            .map(|hit| hit.lightning(&self.light, &ray.direction))
    }

    fn hit_with_ray(&self, ray: &Ray) -> Option<RayHit> {
        if let Some(shape) = self.shapes.first() {
            let intersections = shape.intersect_with(ray);
            if intersections.is_empty() {
                return None;
            } else {
                let first_intersection = intersections[0];
                let hit_point = ray.position(first_intersection);
                let hit = RayHit {
                    point: hit_point,
                    shape: &shape,
                };
                return Some(hit);
            }
        } else {
            None
        }
    }
}
