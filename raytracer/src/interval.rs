use std::f64::INFINITY;

pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Interval {
        Interval {
            min,
            max,
        }
    }

    const fn new_const(min: f64, max: f64) -> Interval {
        Interval {
            min,
            max,
        }
    }

    pub fn size(&self) -> f64 {
        return self.max - self.min;
    }

    pub fn contains(&self, x: f64) -> bool {
        return x >= self.min && x <= self.max;
    }

    pub fn surrounds(&self, x: f64) -> bool {
        return x > self.min && x < self.max;
    }
}

pub const EMPTY: Interval = Interval::new_const(INFINITY, -INFINITY);
pub const UNIVERSE: Interval = Interval::new_const(-INFINITY, INFINITY);