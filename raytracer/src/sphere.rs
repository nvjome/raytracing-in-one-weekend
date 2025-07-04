use std::ops::Range;
use glam::DVec3;
use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray3
};

// #[derive(Debug, Default)]
pub struct Sphere {
    center: DVec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: DVec3, radius: f64, material: Material) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray3, interval: Range<f64>) -> Option<HitRecord> {
        let vect_oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(vect_oc);
        let c = vect_oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        // check range of root(s)
        if !interval.contains(&root) {
            root = (h + sqrtd) / a;
            if !interval.contains(&root) {
                // neither root in valid range
                return None;
            }
        }

        let t = root;
        let point = ray.at(t);
        // record.normal = (record.p - self.center) / self.radius;
        let outward_normal = ( point - self.center) / self.radius;
        // record.set_face_normal(ray, outward_normal);

        let record = HitRecord::with_face_normal(point, outward_normal, t, self.material, ray);

        Some(record)
    }
}