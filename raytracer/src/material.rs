use glam::DVec3;

use crate::{hittable::HitRecord, ray::Ray3, vector_utils};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian {
        albedo: DVec3,
    },
    Metal {
        albedo: DVec3,
        fuzz: f64,
    },
}

pub struct Scattered {
    pub scattered: Ray3,
    pub attenuation: DVec3,
}

impl Material {
    pub fn scatter(self, incident_ray: Ray3, record: HitRecord) -> Option<Scattered> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scattered_direction = record.normal + vector_utils::random_unit_vector();

                if vector_utils::near_zero(scattered_direction) {
                    scattered_direction = record.normal;
                }

                return Some(Scattered {
                    scattered: Ray3 {
                        origin: record.point,
                        direction: scattered_direction,
                    },
                    attenuation: albedo,
                });
            },

            Material::Metal { albedo , fuzz} => {
                let reflected_direction = 
                    vector_utils::reflect(incident_ray.direction, record.normal)
                    .normalize()
                    + (fuzz * vector_utils::random_unit_vector());
                
                let scattered = Ray3::new(record.point, reflected_direction);

                if scattered.direction.dot(record.normal) > 0.0 {
                    return Some(Scattered {
                        scattered,
                        attenuation: albedo,
                    });
                } else {
                    return None;
                }
                
            }
        }
    }
}