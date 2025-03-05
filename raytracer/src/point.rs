use std::ops;
use crate::vector::Vector3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Point3 {
    pub fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 {x, y, z}
    }

    pub fn get_vector_to(&self, point: &Point3) -> Vector3 {
        Vector3 {
            x: point.x - self.x,
            y: point.y - self.y,
            z: point.z - self.z,
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

impl ops::Sub<Vector3> for Point3 {
    type Output = Point3;

    fn sub(self, other: Vector3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Sub<Point3> for Vector3 {
    type Output = Point3;

    fn sub(self, other: Point3) -> Point3 {
        Point3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_vector() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let p = Point3::new(-1.0, 3.0, 10.0);
        assert_eq!(v + p, p + v);
        assert_eq!(p + v, Point3::new(0.0, 5.0, 13.0));
    }

    #[test]
    fn displacement() {
        let p1 = Point3::new(1.0, -2.0, 3.0);
        let p2 = Point3::new(4.0, 5.0, -6.0);
        assert_eq!(p1.get_vector_to(&p2), Vector3::new(3.0, 7.0, -9.0));
    }
}