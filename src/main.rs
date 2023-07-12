use canvas::Canvas;
use color::Color;
use ppm::PpmFile;

mod canvas;
mod color;
mod geometry;
mod ppm;

fn main() {
    let mut canvas = Canvas::new(200, 200);
    canvas.fill(&Color::WHITE);

    for x in 0..canvas.width() {
        for y in 0..canvas.height() {
            if x == y {
                canvas.write_pixel(x, y, &Color(0., 1.0, 0.))
            }
        }
    }

    PpmFile::write("test-output.ppm", &canvas).unwrap();
}
