use drawing::Canvas;
use drawing::Color;
use geometry::Point;
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

    fn scan(&self, canvas_size: usize, mut f: impl FnMut(&CanvasPoint, &Ray) -> ()) {
        for (canvas_point, world_point) in self.generate_world_points(canvas_size) {
            let ray_direction = (world_point - &self.origin).normalize();
            let ray = Ray::new(&self.origin, &ray_direction);
            f(&canvas_point, &ray);
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

fn has_hit(r: &Ray, shape: &Sphere) -> bool {
    !shape.intersect_with(r).is_empty()
}

fn main() {
    let color = Color::new(1., 0., 0.);
    let sphere = Sphere::unit();
    let raycaster = Raycaster::new(Point(0., 0., -5.0), 10.0);

    let mut canvas = Canvas::square(200);
    raycaster.scan(canvas.width(), |canvas_point, ray| {
        if has_hit(&ray, &sphere) {
            canvas.write_pixel(canvas_point.0, canvas_point.1, &color);
        }
    });
    write_ppm("output/test-output.ppm", &canvas).unwrap();
}
