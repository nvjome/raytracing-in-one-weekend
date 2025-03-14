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

    pub fn color(self, depth: i32, world: &impl Hittable) -> DVec3 {
        if depth <= 0 {
            return DVec3::new(0.0, 0.0, 0.0);
        }

        match world.hit(self, 0.001..f64::INFINITY) {
            // Ray hit an object
            Some(record) => { // Hit
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
                // Get random reflection vector from Lambertian distribution (normal vector + random unit vector)
                // let scattered_ray = Self::new(record.point, record.normal + vector_utils::random_unit_vector());
                // Get color of next ray using recursion
                // return 0.5 * Self::color(scattered_ray, depth - 1, world);
            },
            // Ray didn't hit an object
            None => ()
        }

        let unit_dir = self.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
    }
}