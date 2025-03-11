use crate::{hittable::{HitRecord, Hittable}, point::Point3, ray::Ray3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Sphere {
        Sphere {
            center,
            radius: radius.max(0.0),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray3, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let vect_oc = self.center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(vect_oc);
        let c = vect_oc.length_squared() - self.radius*self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (h - sqrtd) / a;

        // check range of root(s)
        if (root >= t_max) || (root <= t_min) {
            root = (h + sqrtd) / a;
            if (root >= t_max) || (root <= t_min) {
                // neither root in valid range
                return false;
            }
        }

        record.t = root;
        record.p = ray.at(record.t);
        // record.normal = (record.p - self.center) / self.radius;
        let outward_normal = (record.p - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);

        return true;
    }
}