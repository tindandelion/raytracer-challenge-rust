use drawing::Canvas;
use drawing::Color;
use geometry::Point;
use geometry::Vector;
use intersect_sphere::Sphere;
use ppm::write_ppm;
use raycaster::Ray;

mod drawing;
mod geometry;
mod intersect_sphere;
mod ppm;
mod raycaster;

type CanvasPoint = (usize, usize);

struct Raycaster {
    origin: Point,
    wall_z: f64,
}

impl Raycaster {
    fn new(origin: Point, wall_z: f64) -> Raycaster {
        Raycaster { origin, wall_z }
    }

    fn scan(&self, canvas_size: usize, mut f: impl FnMut(&Ray, &CanvasPoint) -> ()) {
        for (canvas_point, world_point) in self.generate_world_points(canvas_size) {
            let ray_direction = (world_point - &self.origin).normalize();
            let ray = Ray::new(&self.origin, &ray_direction);
            f(&ray, &canvas_point);
        }
    }

    fn generate_world_points(
        &self,
        canvas_size: usize,
    ) -> impl Iterator<Item = (CanvasPoint, Point)> + '_ {
        let half_wall = self.half_wall_size();
        let pixel_size = (half_wall * 2.) / (canvas_size as f64);

        (0..canvas_size)
            .map(move |y| {
                let world_y = half_wall - pixel_size * (y as f64);
                (y, world_y)
            })
            .flat_map(move |(y, world_y)| {
                (0..canvas_size).map(move |x| {
                    let world_x = -half_wall + pixel_size * (x as f64);
                    let position = Point(world_x, world_y, self.wall_z);
                    ((x, y), position)
                })
            })
    }

    fn half_wall_size(&self) -> f64 {
        let ray_z = &self.origin.2.abs();
        (self.wall_z + ray_z) / ray_z + 0.5
    }
}

fn hit_point(r: &Ray, shape: &Sphere) -> Option<Point> {
    let intersections = shape.intersect_with(r);
    if intersections.is_empty() {
        return None;
    } else {
        let first_point = intersections[0];
        return Some(r.position(first_point));
    }
}

fn get_color_at(pt: &Point, shape: &Sphere, eye_direction: &Vector) -> Color {
    let light_position = Point::new(-10., 10., -10.);
    let normal = shape.normal_at(pt);
    let light_vector = (pt - &light_position).normalize();
    let reflection = normal.reflect(&light_vector);

    let cos_alpha = eye_direction.dot(&reflection);
    let luminosity = cos_alpha.max(0.);
    Color::new(luminosity, 0., 0.) + Color::new(0.1, 0., 0.)
}

fn main() {
    let sphere = Sphere::unit();
    let raycaster = Raycaster::new(Point(0., 0., -5.0), 10.0);

    let mut canvas = Canvas::square(200);
    raycaster.scan(canvas.width(), |ray, canvas_point| {
        if let Some(hit_point) = hit_point(&ray, &sphere) {
            let point_color = get_color_at(&hit_point, &sphere, &(-ray.direction));
            canvas.write_pixel(canvas_point.0, canvas_point.1, &point_color);
        }
    });
    write_ppm("output/test-output.ppm", &canvas).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{geometry::Point, intersect_sphere::Sphere, raycaster::Ray};

    #[test]
    fn calc_lighting_at_point() {
        let shape = Sphere::unit();
        let light_position = Point::new(-3., 3., 0.);
        let eye_position = Point::new(-3., 0., 0.);
        let eye_direction = (Point(0., 0.5, 0.) - &eye_position).normalize();

        let ray = Ray::new(&eye_position, &eye_direction);
        let intersections = shape.intersect_with(&ray);
        let hit_distance = *intersections.first().unwrap();
        let hit_point = ray.position(hit_distance);

        let light_direction = (&hit_point - &light_position).normalize();
        let normal = shape.normal_at(&hit_point);
        let reflection_direction = normal.reflect(&light_direction);
        let cos_alpha = (-eye_direction).dot(&reflection_direction);

        assert_eq!(cos_alpha, 0.);
    }
}
