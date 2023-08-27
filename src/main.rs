use std::f64::consts::PI;

use drawing::Color;

use geometry::Point;

use geometry::Vector;
use ppm::write_ppm;
use raycaster::Camera;
use raycaster::PointLight;

use raycaster::World;
use shapes::Material;
use shapes::Plane;
use shapes::Sphere;

mod drawing;
mod geometry;
mod ppm;
mod raycaster;
mod shapes;

fn create_camera(c_width: usize, c_height: usize) -> Camera {
    Camera::new(c_width, c_height, PI / 3.).with_transform(
        &Point::new(0., 1.5, -5.),
        &Point::new(0., 1., 0.),
        &Vector(0., 1., 0.),
    )
}

fn sphere_material(color: Color) -> Material {
    let mut material = Material::default_with_color(color);
    material.diffuse = 0.7;
    material.specular = 0.3;
    material
}

fn middle_sphere() -> Sphere {
    Sphere::new(Point::new(-0.5, 1., 0.5), 1.0)
        .with_material(sphere_material(Color::new(0.1, 1., 0.5)))
}

fn right_sphere() -> Sphere {
    Sphere::new(Point::new(1.5, 0.5, -0.5), 0.5)
        .with_material(sphere_material(Color::new(0.5, 1., 0.1)))
}

fn left_sphere() -> Sphere {
    Sphere::new(Point::new(-1.5, 0.33, -0.75), 0.33)
        .with_material(sphere_material(Color::new(1., 0.8, 0.1)))
}

fn floor() -> Plane {
    let mut material = Material::default_with_color(Color::new(1., 0.9, 0.9));
    material.specular = 0.;
    Plane::new().with_material(material)
}

fn main() {
    let light = PointLight::new(Color::WHITE, Point::new(-10., 10., -10.));

    let mut world = World::new(light);
    world.add_shape(Box::new(floor()));
    world.add_shape(Box::new(middle_sphere()));
    world.add_shape(Box::new(right_sphere()));
    world.add_shape(Box::new(left_sphere()));

    let canvas = world.render(&create_camera(1024, 512));
    write_ppm("output/test-output.ppm", &canvas).unwrap();
}
