use crate::{ray::Ray3, point::Point3, vector::Vector3};

pub trait Hittable {
    fn hit(&self, ray: &Ray3, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool { false }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
}