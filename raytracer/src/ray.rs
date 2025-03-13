use glam::DVec3;

#[derive(Debug, Default, Copy, Clone)]
pub struct Ray3 {
    pub origin: DVec3,
    pub direction: DVec3
}

impl Ray3 {
    pub fn new(origin: DVec3, direction: DVec3) -> Ray3 {
        Ray3 {origin, direction}
    }

    pub fn at(&self, t: f64) -> DVec3 {
        self.origin + t * self.direction
    }
}