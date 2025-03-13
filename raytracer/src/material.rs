use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray3};

pub enum Material {
    Labertian {
        albedo: DVec3,
    },
    Metal {
        albedo: DVec3,
    },
}

impl Material {
    pub fn scatter(self, ray: Ray3, record: HitRecord) {
        todo!()
    }
}