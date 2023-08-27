use crate::geometry::{Normal, Point, Ray};

use super::Material;

pub trait Shape {
    fn material(&self) -> &Material;
    fn normal_at(&self, pt: &Point) -> Normal;
    fn intersect_with(&self, ray: &Ray) -> Vec<f64>;
}
