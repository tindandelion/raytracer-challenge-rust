use std::f64::consts::PI;

use drawing::Canvas;
use drawing::Color;
use geometry::Point;
use geometry::Transform;
use intersect_sphere::Sphere;
use ppm::write_ppm;
use raycaster::Ray;

mod drawing;
mod geometry;
mod intersect_sphere;
mod ppm;
mod raycaster;

fn half_wall_size(ray_origin: &Point, wall_z: f64) -> f64 {
    let ray_z = ray_origin.2.abs();
    (wall_z + ray_z) / ray_z + 0.5
}

fn has_hit(r: &Ray, shape: &Sphere) -> bool {
    let intersections = shape.intersect_with(r);
    !intersections.is_empty()
}

const CANVAS_SIZE: usize = 200;
const WALL_Z: f64 = 10.0;

fn main() {
    let color = Color::new(1., 0., 0.);
    let ray_origin = Point(0., 0., -5.0);
    let sphere = Sphere::unit();

    let half_wall = half_wall_size(&ray_origin, WALL_Z);
    let pixel_size = (half_wall * 2.) / (CANVAS_SIZE as f64);

    let mut canvas = Canvas::square(CANVAS_SIZE);
    for y in 0..CANVAS_SIZE {
        let world_y = half_wall - pixel_size * (y as f64);
        for x in 0..CANVAS_SIZE {
            let world_x = -half_wall + pixel_size * (x as f64);
            let position = Point(world_x, world_y, WALL_Z);
            let ray_direction = (position - &ray_origin).normalize();
            let r = Ray::new(&ray_origin, &ray_direction);

            if has_hit(&r, &sphere) {
                canvas.write_pixel(x, y, &color);
            }
        }
    }

    write_ppm("output/test-output.ppm", &canvas).unwrap();
}
