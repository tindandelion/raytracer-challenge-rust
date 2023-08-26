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
    shape: &'a Sphere,
    point: Point,
    shape_index: usize,
}

#[derive(Debug, PartialEq)]
struct Intersection(usize, f64);

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
        self.intersections_with(ray).first().map(|inter| {
            let Intersection(shape_index, position) = *inter;
            RayHit {
                point: ray.position(position),
                shape: self.shapes.get(shape_index).unwrap(),
                shape_index,
            }
        })
    }

    fn intersections_with(&self, ray: &Ray<'_>) -> Vec<Intersection> {
        let mut intersections: Vec<Intersection> = self
            .shapes
            .iter()
            .enumerate()
            .flat_map(|(i, shape)| {
                shape
                    .intersect_with(ray)
                    .into_iter()
                    .map(move |v| Intersection(i, v))
            })
            .collect();

        intersections.sort_by(|a, b| a.1.total_cmp(&b.1));
        intersections
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
