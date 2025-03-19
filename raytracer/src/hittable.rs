use std::ops::Range;
use glam::DVec3;
use crate::{material::Material, ray::Ray3};

pub trait Hittable: Send + Sync {
    #[allow(unused_variables)]
    fn hit(&self, ray: Ray3, interval: Range<f64>) -> Option<HitRecord> { None }
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub point: DVec3,
    pub normal: DVec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn with_face_normal(point: DVec3, normal: DVec3, t: f64, material: Material, ray: Ray3, ) -> HitRecord {
        let (front_face, normal) = HitRecord::calculate_face_normal(ray, normal);
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }

    fn calculate_face_normal(ray: Ray3, outward_normal: DVec3) -> (bool, DVec3) {
        // Dot product is negative if ray comes from outside (points agains normal),
        // and positive if ray comes from inside (points with normal)
        let front_face = ray.direction.dot(outward_normal) < 0.0;
        let normal = match front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        return (front_face, normal);
    }
}

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::<Box<dyn Hittable>>::new(),
        }
    }

    pub fn new_with(object: Box<dyn Hittable>) -> HittableList {
        HittableList {
            objects: vec![object],
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray3, interval: Range<f64>) -> Option<HitRecord> {
        let mut closest_record: Option<HitRecord> = None;
        let mut closest_t_so_far = interval.end;

        // Loop through all hittable objects in Self::objects
        for object in self.objects.iter() {
            // Check for a hit between the start and the last closest hit
            match object.hit(ray, interval.start..closest_t_so_far) {
                Some(record) => {
                    // Save record as the next closest hit
                    closest_record = Some(record);
                    closest_t_so_far = record.t;
                },
                None => {
                    // No hit, so continue
                }
            }
        };

        return closest_record;
    }
}