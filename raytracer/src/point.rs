use std::ops;
use crate::vector::Vector3;

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {x, y, z}
    }

    pub fn displacement_vector(&self, point: &Point3) -> Vector3 {
        Vector3 {
            x: point.x - self.x,
            y: point.y - self.y,
            z: point.y - self.y,
        }
    }
}

impl ops::Add<Vector3> for Point3 {
    type Output = Point3;

    fn add(self, other: Vector3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Add<Point3> for Vector3 {
    type Output = Point3;

    fn add(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}