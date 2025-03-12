use std::ops;
use rand::{rngs::ThreadRng, Rng};

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
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

// Overload operator functions

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, other: Vector3) -> Self::Output {
        Vector3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Neg for Vector3 {
    type Output = Vector3;

    fn neg(self) -> Self::Output {
        Vector3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl ops::Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, other: f64) -> Self::Output {
        Vector3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl ops::Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, other: Vector3) -> Self::Output {
        Vector3::mul(other, self)
    }
}

impl ops::Div<f64> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f64) -> Self::Output {
        Vector3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

// Some required generator functions, which don't need to be within the Vector3 struct.
// These require the calling code to pass a ThreadRng reference, which is cleaner than
// giving all Vector3s a ThreadRng instance. Calling code can reuse one ThreadRng.
pub fn random(rng: &mut ThreadRng) -> Vector3 {
    Vector3 {
        x: rng.random(),
        y: rng.random(),
        z: rng.random(),
    }
}

pub fn random_bounded(min: f64, max: f64, rng: &mut ThreadRng) -> Vector3 {
    Vector3 {
        x: rng.random_range(min..=max),
        y: rng.random_range(min..=max),
        z: rng.random_range(min..=max),
    }
}

pub fn random_unit(rng: &mut ThreadRng) -> Vector3 {
    loop {
        let p = random_bounded(-1.0, 1.0, rng);
        let lensq = p.length_squared();
        if 1e-160 < lensq && lensq <= 1.0 {
            break p / lensq.sqrt();
        }
    }
}

pub fn random_unit_in_hemisphere(rng: &mut ThreadRng, normal: Vector3) -> Vector3 {
    let unit = random_unit(rng);
    if unit.dot(normal) > 0.0 {
        return unit;
    } else {
        return -unit;
    }
}

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