use overload::overload;
use std::ops;

#[derive(PartialEq, Debug)]
pub struct Color(pub f64, pub f64, pub f64);

overload!((a: ?Color) + (b: ?Color) -> Color { Color(a.0 + b.0, a.1 + b.1, a.2 + b.2) });
overload!((a: ?Color) - (b: ?Color) -> Color { Color(a.0 - b.0, a.1 - b.1, a.2 - b.2) });
overload!((a: ?Color) * (c: f64) -> Color { Color(a.0 * c, a.1 * c, a.2 * c) });
overload!((a: ?Color) * (b: ?Color) -> Color { Color(a.0 * b.0, a.1 * b.1, a.2 * b.2) });

impl Color {
    pub const BLACK: Color = Color(0., 0., 0.);

    pub fn clone(&self) -> Color {
        Color(self.0, self.1, self.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_arithmetic() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);

        assert_colors_eq(&c1 + &c2, Color(1.6, 0.7, 1.0));
        assert_colors_eq(&c1 - &c2, Color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color(0.2, 0.3, 0.4);
        assert_colors_eq(c * 2., Color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_color_by_color() {
        let c1 = Color(1., 0.2, 0.4);
        let c2 = Color(0.9, 1., 0.1);

        assert_colors_eq(c1 * c2, Color(0.9, 0.2, 0.04));
    }

    fn assert_colors_eq(c1: Color, c2: Color) {
        let delta = 0.0001;
        assert!(c1.0 - c2.0 < delta, "R");
        assert!(c1.1 - c2.1 < delta, "G");
        assert!(c1.2 - c2.2 < delta, "B");
    }
}
