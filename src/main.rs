use drawing::Canvas;
use drawing::Color;
use geometry::Normal;
use geometry::Point;
use geometry::UnitVector;
use geometry::Vector;
use intersect_sphere::Sphere;
use ppm::write_ppm;
use raycaster::Material;
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
    let light_vector = (pt - &light_position).normalize();
    let normal = shape.normal_at(pt);
    Material::default().lighting(&light_vector, eye_direction, &normal)
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
