use std::ops;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vector3 {
        Vector3 {x, y, z}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn dot(self, rhs: Vector3) -> f64 {
        self.x*rhs.x + self.y*rhs.y + self.z*rhs.z
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.y*rhs.z - self.z*rhs.y,
            y: self.z*rhs.x - self.x*rhs.z,
            z: self.x*rhs.y - self.y*rhs.x,
        }
    }

    pub fn unit(&self) -> Vector3 {
        *self / self.length()
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/* impl ops::AddAssign<Vector3> for Vector3 {
    fn add_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
} */

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Vector3 {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/* impl ops::SubAssign<Vector3> for Vector3 {
    fn sub_assign(&mut self, other: Vector3) {
        *self = Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
} */

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Vector3 {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Vector3 {
        Vector3::mul(other, self)
    }
}

/* impl ops::MulAssign<f64> for Vector3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
} */

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/* impl ops::DivAssign<f64> for Vector3 {
    fn div_assign(&mut self, rhs: f64) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
    }
} */

// pub type Point3 = Vector3;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length() {
        let v = Vector3::new(1.0, 1.0, 1.0);
        assert_eq!(v.length(), (3.0_f64).sqrt());
    }

    #[test]
    fn add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(-1.0, -2.0, -3.0);
        assert_eq!(v1 + v2, Vector3::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn dot() {
        let v1 = Vector3::new(-6.0, 8.0, 0.0);
        let v2 = Vector3::new(5.0, 12.0, 0.0);
        assert_eq!(v1.dot(v2), v2.dot(v1));
        assert_eq!(v1.dot(v2), 66.0);
    }
}