use std::f64::consts::PI;

use canvas::Canvas;
use color::Color;
use geometry::Point;
use ppm::write_ppm;
use transform::Transform;

mod canvas;
mod color;
mod geometry;
mod ppm;
mod transform;

const CLOCK_RADIUS: usize = 100;
const CANVAS_SIZE: usize = CLOCK_RADIUS * 2 + 20;

fn calc_hour_position(hour: i32) -> Point {
    let angle = PI / 6. * (hour as f64);
    let transform = Transform::rotate_z(angle);

    let twelve = Point(0., 1., 0.);
    transform * twelve
}

fn to_canvas_coord(v: f64) -> usize {
    (v * (CLOCK_RADIUS as f64) + (CANVAS_SIZE as f64) / 2.0).round() as usize
}

fn to_canvas_point(hour_pos: &Point) -> (usize, usize) {
    let Point(x, y, _) = hour_pos;
    (to_canvas_coord(*x), to_canvas_coord(-y))
}

fn main() {
    let hour_mark_color = Color::BLACK;
    let mut canvas = Canvas::new(CANVAS_SIZE, CANVAS_SIZE);
    canvas.fill(&Color::WHITE);

    for hour in 0..12 {
        let hour_pos = calc_hour_position(hour);
        let (x, y) = to_canvas_point(&hour_pos);
        canvas.write_pixel(x, y, &hour_mark_color)
    }

    write_ppm("output/test-output.ppm", &canvas).unwrap();
}
