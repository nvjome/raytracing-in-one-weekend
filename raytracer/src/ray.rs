use crate::{point::Point3, vector::Vector3};

#[derive(Debug, Copy, Clone)]
pub struct Ray3 {
    pub origin: Point3,
    pub direction: Vector3
}

impl Ray3 {
    pub fn new(origin: Point3, direction: Vector3) -> Ray3 {
        Ray3 {origin, direction}
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}