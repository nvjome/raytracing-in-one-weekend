use crate::{ray::Ray3, point::Point3, vector::Vector3};

pub trait Hittable {
    #[allow(unused_variables)]
    fn hit(&self, ray: Ray3, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool { false }
}

#[derive(Copy, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vector3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vector3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, ray: Ray3, outward_normal: Vector3) {
        // Dot product is negative is ray comes from outside (points agains normal),
        // and positive if ray comes from inside (points with normal)
        self.front_face = ray.direction.dot(outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal,
            false => -1.0 * outward_normal,
        }
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
    fn hit(&self, ray: Ray3, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything: bool = false;
        let mut closest_so_far: f64 = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, closest_so_far, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_record.t;
                *record = temp_record;
            }
        }
        
        return hit_anything;
    }
}