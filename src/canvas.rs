use crate::color::Color;

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let mut pixels: Vec<Color> = Vec::with_capacity(width * height);
        for _ in 0..pixels.capacity() {
            pixels.push(Color::BLACK)
        }
        Canvas {
            width,
            height,
            pixels,
        }
    }

    fn write_pixel(&mut self, x: usize, y: usize, color: &Color) {
        self.pixels[y * self.width + x] = color.clone();
    }

    fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x].clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn create_canvas() {
        let cnv = Canvas::new(10, 20);

        assert_eq!(cnv.width, 10);
        assert_eq!(cnv.height, 20);
        for pix in cnv.pixels {
            assert_eq!(pix, Color::BLACK)
        }
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color(1., 0., 0.);

        canvas.write_pixel(2, 3, &red);
        assert_eq!(canvas.pixel_at(2, 3), red);
    }
}
