use std::f64::consts::PI;

use drawing::Canvas;
use drawing::Color;
use geometry::Point;
use geometry::UnitVector;

use ppm::write_ppm;
use raycaster::Camera;
use raycaster::PointLight;
use raycaster::Ray;
use shapes::Material;
use shapes::Sphere;

mod drawing;
mod geometry;
mod ppm;
mod raycaster;
mod shapes;

fn scan(canvas_size: usize, mut f: impl FnMut(&Ray, usize, usize) -> ()) {
    let camera = Camera::new(canvas_size, canvas_size, PI / 2.);
    for y in 0..canvas_size {
        for x in 0..canvas_size {
            camera.cast_ray_at(x, y, |r| f(&r, x, y));
        }
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

fn get_color_at(pt: &Point, shape: &Sphere, ray_direction: &UnitVector) -> Color {
    let color = Color::new(1., 0.2, 1.);
    let light = PointLight::new(Color::WHITE, Point::new(-10., 10., 10.));
    let normal = shape.normal_at(pt);
    Material::default_with_color(color).lighting(&light, &pt, &(ray_direction.flip()), &normal)
}

fn main() {
    let sphere = Sphere::new(Point::new(0., 0., -0.5), 0.25);
    let mut canvas = Canvas::square(512);

    scan(canvas.width(), |ray, px, py| {
        if let Some(hit_point) = hit_point(&ray, &sphere) {
            let point_color = get_color_at(&hit_point, &sphere, &ray.direction);
            canvas.write_pixel(px, py, &point_color);
        }
    });
    write_ppm("output/test-output.ppm", &canvas).unwrap();
}
