use std::f64::consts::PI;

use drawing::Canvas;
use drawing::Color;

use geometry::Point;

use geometry::Vector;
use ppm::write_ppm;
use raycaster::Camera;
use raycaster::PointLight;
use raycaster::Ray;
use raycaster::ViewTransform;
use raycaster::World;
use shapes::Material;
use shapes::Sphere;

mod drawing;
mod geometry;
mod ppm;
mod raycaster;
mod shapes;

fn scan(canvas_size: usize, mut f: impl FnMut(&Ray, usize, usize) -> ()) {
    let mut camera = Camera::new(canvas_size, canvas_size, PI / 2.);
    let camera_pos = Point::new(0., 0., -1.75);
    let camera_dir = Point::new(0., 0., 1.);
    let camera_up = Vector(0., 1., 0.);

    camera.set_transform(ViewTransform::new(&camera_pos, &camera_dir, &camera_up));
    for y in 0..canvas_size {
        for x in 0..canvas_size {
            camera.cast_ray_at(x, y, |r| f(&r, x, y));
        }
    }
}

fn material(color: Color) -> Material {
    Material::default_with_color(color)
}

fn main() {
    let big_sphere = Sphere::unit().with_material(material(Color::new(1., 0.2, 1.)));
    let small_sphere = Sphere::new(Point::new(0., 0., -0.5), 0.25)
        .with_material(material(Color::new(0.2, 0.2, 1.)));
    let light = PointLight::new(Color::WHITE, Point::new(10., 10., -10.));

    let mut world = World::new(light);
    world.add_shape(small_sphere);
    world.add_shape(big_sphere);

    let mut canvas = Canvas::square(512);
    scan(canvas.width(), |ray, px, py| {
        let point_color = world.get_color(&ray).unwrap_or(Color::BLACK);
        canvas.write_pixel(px, py, &point_color);
    });
    write_ppm("output/test-output.ppm", &canvas).unwrap();
}
