use crate::{
    drawing::Color,
    geometry::{Normal, Point},
    shapes::Sphere,
};

use super::{PointLight, Ray};

pub struct World {
    pub light: PointLight,
    pub shape: Sphere,
}

struct RayHit<'a> {
    point: Point,
    shape: &'a Sphere,
}

impl RayHit<'_> {
    fn normal(&self) -> Normal {
        self.shape.normal_at(&self.point)
    }
}

impl World {
    pub fn get_color(&self, ray: &Ray) -> Option<Color> {
        self.hit_with_ray(&ray).map(|hit| {
            let normal = hit.normal();
            let eye_direction = ray.direction.flip();
            hit.shape
                .material()
                .lighting(&self.light, &hit.point, &eye_direction, &normal)
        })
    }

    fn hit_with_ray(&self, ray: &Ray) -> Option<RayHit> {
        let intersections = self.shape.intersect_with(ray);
        if intersections.is_empty() {
            return None;
        } else {
            let first_intersection = intersections[0];
            let hit_point = ray.position(first_intersection);
            let hit = RayHit {
                point: hit_point,
                shape: &self.shape,
            };
            return Some(hit);
        }
    }
}
