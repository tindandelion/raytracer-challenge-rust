use crate::{
    drawing::{Canvas, Color},
    geometry::{Normal, Point, Ray, UnitVector},
    shapes::Shape,
};

use super::{Camera, PointLight};

pub struct World {
    light: PointLight,
    shapes: Vec<Box<dyn Shape>>,
}

struct RayHit<'a> {
    shape: &'a Box<dyn Shape>,
    shape_index: usize,
    is_shadowed: bool,
    point: Point,
    eye_direction: UnitVector,
    normal: Normal,
}

#[derive(Clone, Copy)]
struct Intersection(usize, f64);

impl RayHit<'_> {
    fn lightning(&self, light: &PointLight) -> Color {
        self.shape.material().lighting(
            light,
            &self.point,
            &self.eye_direction,
            &self.normal,
            self.is_shadowed,
        )
    }
}

impl World {
    pub fn new(light: PointLight) -> World {
        World {
            light,
            shapes: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) -> usize {
        self.shapes.push(shape);
        self.shapes.len() - 1
    }

    pub fn render(&self, camera: &Camera) -> Canvas {
        let mut canvas = Canvas::new(camera.h_size(), camera.v_size());
        camera.scan_space(|ray, px, py| {
            let point_color = self.get_color(&ray).unwrap_or(Color::BLACK);
            canvas.write_pixel(px, py, &point_color);
        });
        canvas
    }

    fn get_color(&self, ray: &Ray) -> Option<Color> {
        self.hit_with_ray(&ray)
            .map(|hit| hit.lightning(&self.light))
    }

    fn is_shadowed(&self, point: &Point) -> bool {
        let distance_to_light = self.light.distance_from(point);
        let direction_to_light = self.light.direction_from(point);
        let shadow_ray = Ray::new(point, direction_to_light);
        self.first_intersection_with(&shadow_ray)
            .map(|Intersection(_, distance)| distance < distance_to_light)
            .unwrap_or(false)
    }

    fn hit_with_ray(&self, ray: &Ray) -> Option<RayHit> {
        self.first_intersection_with(ray)
            .map(|Intersection(shape_index, pos)| {
                let shape = self.shapes.get(shape_index).unwrap();
                let point = ray.position(pos);
                let mut normal = shape.normal_at(&point);
                let eye_direction = ray.direction.flip();
                let is_inside = normal.dot(&eye_direction) < 0.;
                if is_inside {
                    normal = normal.flip()
                }
                let is_shadowed = self.is_shadowed(&normal.over_point(&point));

                RayHit {
                    shape,
                    shape_index,
                    point,
                    normal,
                    eye_direction,
                    is_shadowed,
                }
            })
    }

    fn first_intersection_with(&self, ray: &Ray<'_>) -> Option<Intersection> {
        self.intersect_with(ray).first().copied()
    }

    fn intersect_with(&self, ray: &Ray<'_>) -> Vec<Intersection> {
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
            .filter(|inter| inter.1 >= 0.)
            .collect();

        intersections.sort_by(|a, b| a.1.total_cmp(&b.1));
        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::World;

    use crate::{
        drawing::Color,
        geometry::{Normal, Point, Ray, UnitVector, Vector},
        raycaster::PointLight,
        shapes::Sphere,
    };

    const LIGHT_POS: Point = Point::new(-10., 10., -10.);
    const LIGHT: PointLight = PointLight::new(Color::WHITE, LIGHT_POS);

    fn world_with_unit_sphere() -> World {
        let mut world = World::new(LIGHT);
        world.add_shape(Box::new(Sphere::new(Point::ZERO, 1.)));
        world
    }

    mod ray_intersections {
        use super::*;

        const RAY_ORIGIN: Point = Point::new(0., 0., -5.);
        const RAY: Ray = Ray::new(&RAY_ORIGIN, UnitVector::Z);

        #[test]
        fn empty_world_produces_no_intersections() {
            let empty_world = World::new(LIGHT);

            let intersections = empty_world.intersect_with(&RAY);
            assert!(intersections.is_empty())
        }

        #[test]
        fn intersect_world_with_ray() {
            let mut world = world_with_unit_sphere();
            world.add_shape(Box::new(Sphere::new(Point::ZERO, 0.5)));

            let intersections = world.intersect_with(&RAY);
            let positions: Vec<f64> = intersections.into_iter().map(|i| i.1).collect();
            assert_eq!(positions, vec![4., 4.5, 5.5, 6.])
        }

        #[test]
        fn ignore_intersections_in_negative_directions() {
            let world = world_with_unit_sphere();
            let ray_from_inside = Ray::new(&Point::ZERO, UnitVector::Z);

            let intersections = world.intersect_with(&ray_from_inside);
            let positions: Vec<f64> = intersections.into_iter().map(|i| i.1).collect();
            assert_eq!(positions, vec![1.0])
        }

        #[test]
        fn ray_hits_object_from_outside() {
            let world = world_with_unit_sphere();
            let hit = world.hit_with_ray(&RAY).unwrap();

            assert_eq!(hit.shape_index, 0);
            assert_eq!(hit.point, Point::new(0., 0., -1.));
            assert_eq!(hit.eye_direction, Vector(0., 0., -1.).normalize());
            assert_eq!(hit.normal, Normal::from(&Vector(0., 0., -1.)));
        }

        #[test]
        fn ray_hits_object_from_inside_flips_the_normal() {
            let world = world_with_unit_sphere();
            let ray_from_inside = Ray::new(&Point::ZERO, UnitVector::Z);
            let hit = world.hit_with_ray(&ray_from_inside).unwrap();

            assert_eq!(hit.shape_index, 0);
            assert_eq!(hit.point, Point::new(0., 0., 1.));
            assert_eq!(hit.eye_direction, Vector(0., 0., -1.).normalize());
            assert_eq!(hit.normal, Normal::from(&Vector(0., 0., -1.)))
        }

        #[test]
        fn cast_ray_hits_closest_object() {
            let mut world = World::new(LIGHT);

            let front_shape = Sphere::new(Point::new(0., 0., -4.), 0.1);
            let back_shape = Sphere::new(Point::new(0., 0., -1.), 0.1);

            world.add_shape(Box::new(back_shape));
            let front_shape_index = world.add_shape(Box::new(front_shape));
            let hit = world.hit_with_ray(&RAY).unwrap();

            assert_eq!(hit.shape_index, front_shape_index);
        }
    }

    mod shadowing {
        use super::*;

        #[test]
        fn not_shadowed_when_nothing_is_collinear_with_point_and_light() {
            let world = world_with_unit_sphere();
            let test_point = Point::new(0., 10., 0.);

            assert!(!world.is_shadowed(&test_point));
        }

        #[test]
        fn not_shadowed_when_object_behind_the_light() {
            let world = world_with_unit_sphere();
            let test_point = Point::new(-20., 20., -20.);

            assert!(!world.is_shadowed(&test_point));
        }

        #[test]
        fn not_shadowed_when_object_behind_the_point() {
            let world = world_with_unit_sphere();
            let test_point = Point::new(-20., 20., -20.);

            assert!(!world.is_shadowed(&test_point));
        }

        #[test]
        fn shadowed_when_object_between_point_and_light() {
            let world = world_with_unit_sphere();
            let test_point = Point::new(10., -10., 10.);

            assert!(world.is_shadowed(&test_point));
        }
    }
}
