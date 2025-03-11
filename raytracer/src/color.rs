use std::ops;

use crate::vector::Vector3;

#[derive(Debug, Default, Copy, Clone)]
pub struct ColorRGB {
    pub r: f64,
    pub g: f64,
    pub b: f64
}

impl ColorRGB {
    pub fn new(r: f64, g: f64, b: f64) -> ColorRGB {
        ColorRGB {
            r: r.clamp(0.0, 1.0),
            g: g.clamp(0.0, 1.0),
            b: b.clamp(0.0, 1.0)}
    }
}

impl ops::Add for ColorRGB {
    type Output = ColorRGB;

    fn add(self, other: ColorRGB) -> ColorRGB {
        ColorRGB {
            r: (self.r + other.r).clamp(0.0, 1.0),
            g: (self.g + other.g).clamp(0.0, 1.0),
            b: (self.b + other.b).clamp(0.0, 1.0),
        }
    }
}

impl ops::Add<ColorRGB> for Vector3 {
    type Output = ColorRGB;

    fn add(self, other: ColorRGB) -> ColorRGB {
        ColorRGB {
            r: (self.x + other.r).clamp(0.0, 1.0),
            g: (self.y + other.g).clamp(0.0, 1.0),
            b: (self.z + other.b).clamp(0.0, 1.0),
        }
    }
}

impl ops::Mul<f64> for ColorRGB {
    type Output = ColorRGB;

    fn mul(self, other: f64) -> ColorRGB {
        ColorRGB {
            r: (self.r * other).clamp(0.0, 1.0),
            g: (self.g * other).clamp(0.0, 1.0),
            b: (self.b * other).clamp(0.0, 1.0),
        }
    }
}

impl ops::Mul<ColorRGB> for f64 {
    type Output = ColorRGB;

    fn mul(self, other: ColorRGB) -> ColorRGB {
        ColorRGB::mul(other, self)
    }
}

pub fn ppm_preamble(x: i32, y: i32) {
    println!("P3");
    println!("{} {}", x, y);
    println!("255");
}

pub fn ppm_write_pixel(color: ColorRGB) {
    let r = (color.r * 255.99) as u8;
    let g = (color.g * 255.99) as u8;
    let b = (color.b * 255.99) as u8;

    println!("{} {} {}", r, g, b);
}