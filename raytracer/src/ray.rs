use glam::DVec3;
use crate::hittable::Hittable;

#[derive(Copy, Clone)]
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

    pub fn color(self, depth: i32, world: &dyn Hittable) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }

        if let Some(record) = world.hit(self, 0.001..f64::INFINITY) { // Hit
            match record.material.scatter(self, record) {
                // Ray scattered
                Some(scattered) => {
                    return scattered.attenuation * Self::color(scattered.scattered, depth - 1, world);
                },
                // Ray absorbed
                None => {
                    return DVec3::new(0.0, 0.0, 0.0);
                }
            }
        }

        let unit_dir = self.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0)
    }
}