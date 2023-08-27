use crate::{
    drawing::{Canvas, Color},
    geometry::{Normal, Point, Ray, UnitVector},
    shapes::Sphere,
};

use super::{Camera, PointLight};

pub struct World {
    light: PointLight,
    shapes: Vec<Sphere>,
}

struct RayHit<'a> {
    shape: &'a Sphere,
    shape_index: usize,
    is_inside: bool,
    point: Point,
    eye_direction: UnitVector,
    normal: Normal,
}

struct Intersection(usize, f64);

impl RayHit<'_> {
    fn lightning(&self, light: &PointLight) -> Color {
        self.shape
            .material()
            .lighting(light, &self.point, &self.eye_direction, &self.normal)
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

    fn hit_with_ray(&self, ray: &Ray) -> Option<RayHit> {
        self.intersect_with(ray).first().map(|inter| {
            let Intersection(shape_index, position) = *inter;
            let shape = self.shapes.get(shape_index).unwrap();
            let point = ray.position(position);
            let mut normal = shape.normal_at(&point);
            let eye_direction = ray.direction.flip();
            let is_inside = normal.dot(eye_direction.v()) < 0.;
            if is_inside {
                normal = normal.flip()
            }

            RayHit {
                shape,
                shape_index,
                point,
                normal,
                eye_direction,
                is_inside,
            }
        })
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

    use crate::{
        drawing::Color,
        geometry::{Normal, Point, Ray, UnitVector, Vector},
        raycaster::PointLight,
        shapes::Sphere,
    };

    use super::World;

    const RAY_ORIGIN: Point = Point::new(0., 0., -5.);
    const RAY: Ray = Ray::new(&RAY_ORIGIN, UnitVector::Z);
    const LIGHT: PointLight = PointLight::new(Color::WHITE, Point::ZERO);

    #[test]
    fn empty_world_produces_no_hits() {
        let empty_world = World::new(LIGHT);
        let hit = empty_world.hit_with_ray(&RAY);

        assert!(hit.is_none())
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
    fn intersect_world_with_ray() {
        let mut world = world_with_unit_sphere();
        world.add_shape(Sphere::new(Point::ZERO, 0.5));

        let intersections = world.intersect_with(&RAY);
        let positions: Vec<f64> = intersections.into_iter().map(|i| i.1).collect();
        assert_eq!(positions, vec![4., 4.5, 5.5, 6.])
    }

    #[test]
    fn ray_hits_object_from_outside() {
        let world = world_with_unit_sphere();
        let hit = world.hit_with_ray(&RAY).unwrap();

        assert_eq!(hit.shape_index, 0);
        assert_eq!(hit.is_inside, false);
        assert_eq!(hit.point, Point::new(0., 0., -1.));
        assert_eq!(hit.eye_direction, Vector(0., 0., -1.).normalize());
        assert_eq!(hit.normal, Normal::from(&Vector(0., 0., -1.)));
    }

    #[test]
    fn ray_hits_object_from_inside() {
        let world = world_with_unit_sphere();
        let ray_from_inside = Ray::new(&Point::ZERO, UnitVector::Z);
        let hit = world.hit_with_ray(&ray_from_inside).unwrap();

        assert_eq!(hit.shape_index, 0);
        assert_eq!(hit.is_inside, true);
        assert_eq!(hit.point, Point::new(0., 0., 1.));
        assert_eq!(hit.eye_direction, Vector(0., 0., -1.).normalize());
        assert_eq!(hit.normal, Normal::from(&Vector(0., 0., -1.)))
    }

    #[test]
    fn cast_ray_hits_closest_object() {
        let mut world = World::new(LIGHT);

        let front_shape = Sphere::new(Point::new(0., 0., -4.), 0.1);
        let back_shape = Sphere::new(Point::new(0., 0., -1.), 0.1);

        world.add_shape(back_shape);
        let front_shape_index = world.add_shape(front_shape);
        let hit = world.hit_with_ray(&RAY).unwrap();

        assert_eq!(hit.shape_index, front_shape_index);
    }

    fn world_with_unit_sphere() -> World {
        let mut world = World::new(LIGHT);
        world.add_shape(Sphere::new(Point::ZERO, 1.));
        world
    }
}
