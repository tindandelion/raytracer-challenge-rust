use overload::overload;
use std::ops;

use crate::geometry::Vector;

#[derive(PartialEq, Debug)]
pub struct Color {
    value: Vector,
}

overload!((a: ?Color) + (b: ?Color) -> Color { Color { value: &a.value + &b.value } });
overload!((a: ?Color) - (b: ?Color) -> Color { Color { value: &a.value - &b.value } });
overload!((a: ?Color) * (c: f64) -> Color { Color { value: &a.value * c } });
overload!((a: ?Color) * (b: ?Color) -> Color { Color { value: mul_pairwise(&a.value, &b.value) } });

impl Color {
    pub const BLACK: Color = Color {
        value: Vector(0., 0., 0.),
    };
    pub const WHITE: Color = Color {
        value: Vector(1., 1., 1.),
    };

    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            value: Vector(r, g, b),
        }
    }

    pub fn r(&self) -> f64 {
        self.value.0
    }

    pub fn g(&self) -> f64 {
        self.value.1
    }

    pub fn b(&self) -> f64 {
        self.value.2
    }

    pub fn to_a(&self) -> [f64; 3] {
        [self.value.0, self.value.1, self.value.2]
    }

    pub fn clone(&self) -> Color {
        Color {
            value: Vector(self.value.0, self.value.1, self.value.2),
        }
    }
}

fn mul_pairwise(a: &Vector, b: &Vector) -> Vector {
    Vector(a.0 * b.0, a.1 * b.1, a.2 * b.2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_arithmetic() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        assert_colors_eq(&c1 + &c2, Color::new(1.6, 0.7, 1.0));
        assert_colors_eq(&c1 - &c2, Color::new(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiply_color_by_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);
        assert_colors_eq(c * 2., Color::new(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiply_color_by_color() {
        let c1 = Color::new(1., 0.2, 0.4);
        let c2 = Color::new(0.9, 1., 0.1);

        assert_colors_eq(c1 * c2, Color::new(0.9, 0.2, 0.04));
    }

    fn assert_colors_eq(c1: Color, c2: Color) {
        let allowed_delta = 0.0001;
        let color_delta = (c1.value - c2.value).magnitude();
        assert!(color_delta < allowed_delta);
    }
}
