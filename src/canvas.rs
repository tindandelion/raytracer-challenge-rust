use crate::color::Color;

struct Canvas {
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let mut pixels: Vec<Vec<Color>> = Vec::with_capacity(height);
        for _ in 0..height {
            let mut row: Vec<Color> = Vec::with_capacity(width);
            for _ in 0..width {
                row.push(Color::BLACK)
            }
            pixels.push(row)
        }

        Canvas { pixels }
    }

    fn width(&self) -> usize {
        self.pixels[0].len()
    }

    fn height(&self) -> usize {
        self.pixels.len()
    }

    fn pixels(&self) -> &Vec<Vec<Color>> {
        &self.pixels
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::Color;

    #[test]
    fn create_canvas() {
        let cnv = Canvas::new(10, 20);

        assert_eq!(cnv.width(), 10);
        assert_eq!(cnv.height(), 20);
        for row in cnv.pixels() {
            for pix in row {
                assert_eq!(pix, &Color(0., 0., 0.))
            }
        }
    }

    #[test]
    fn write_pixel_to_canvas() {
        let mut canvas = Canvas::new(10, 20);
        let red = Color(1., 0., 0.);
        todo!("Not yet implemented");
    }
}
