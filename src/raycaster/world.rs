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
    shape_index: usize,
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

    pub fn add_shape(&mut self, shape: Sphere) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
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
                    shape_index: 0,
                };
                return Some(hit);
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        drawing::Color,
        geometry::{Point, UnitVector},
        raycaster::{PointLight, Ray},
        shapes::Sphere,
    };

    use super::World;

    const RAY_ORIGIN: Point = Point::new(0., 0., -100.);
    const RAY: Ray = Ray::new(&RAY_ORIGIN, UnitVector::Z);
    const LIGHT: PointLight = PointLight::new(Color::WHITE, Point::ZERO);

    #[test]
    fn empty_world_produces_no_hits() {
        let empty_world = World::new(LIGHT);
        let hit = empty_world.hit_with_ray(&RAY);

        assert!(hit.is_none())
    }

    #[test]
    fn cast_ray_at_single_object_world() {
        let mut world = World::new(LIGHT);

        let shape_index = world.add_shape(Sphere::unit());
        let hit = world.hit_with_ray(&RAY).unwrap();

        assert_eq!(hit.shape_index, shape_index)
    }

    #[test]
    fn cast_ray_hits_closest_object() {
        let mut world = World::new(LIGHT);

        let front_shape = Sphere::new(Point::new(0., 0., -10.), 0.1);
        let back_shape = Sphere::new(Point::new(0., 0., -1.), 0.1);

        world.add_shape(back_shape);
        let front_shape_index = world.add_shape(front_shape);
        let hit = world.hit_with_ray(&RAY).unwrap();

        assert_eq!(hit.shape_index, front_shape_index);
    }
}
