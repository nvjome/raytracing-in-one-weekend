use glam::DVec3;
use rand::Rng;

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
    Dielectric  {
        refraction_index: f64,
    }
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

                Some(Scattered {
                    scattered: Ray3 {
                        origin: record.point,
                        direction: scattered_direction,
                    },
                    attenuation: albedo,
                })
            },

            Material::Metal { albedo , fuzz} => {
                let reflected_direction = 
                    reflect(incident_ray.direction, record.normal)
                    .normalize()
                    + (fuzz * vector_utils::random_unit_vector());
                
                let scattered = Ray3::new(record.point, reflected_direction);

                if scattered.direction.dot(record.normal) > 0.0 {
                    Some(Scattered {
                        scattered,
                        attenuation: albedo,
                    })
                } else {
                    None
                }
                
            },

            Material::Dielectric { refraction_index } => {
                let mut rng = rand::rng();

                let attenuation = DVec3::new(1.0, 1.0, 1.0);
                let refraction_index_corrected = match record.front_face {
                    true => refraction_index.recip(),
                    false => refraction_index,
                };
                let unit_direction = incident_ray.direction.normalize();

                // Prep to check if ray can be refracted
                let cos_theta = (-unit_direction).dot(record.normal).min(1.0);
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

                let cannot_refract = refraction_index_corrected * sin_theta > 1.0;

                let direction = if cannot_refract || (reflectance(cos_theta, refraction_index_corrected) > rng.random_range(0.0..1.0)) {
                    // No refraction solution, so reflect
                    reflect(unit_direction, record.normal)
                } else {
                    // Refract
                    refract(unit_direction, record.normal, refraction_index_corrected)
                };

                Some(Scattered {
                    scattered: Ray3 {
                        origin: record.point,
                        direction,
                    },
                    attenuation,
                })
            },
        }
    }
}

pub fn reflect(vector: DVec3, normal: DVec3) -> DVec3 {
    vector - 2.0 * vector.dot(normal) * normal
}

pub fn refract(vector: DVec3, normal: DVec3, refraction_ratio: f64) -> DVec3 {
    let cos_theta = (-vector).dot(normal).min(1.0);
    let r_perpendicular = refraction_ratio * (vector + cos_theta * normal);
    let r_parallel = -(1.0 - r_perpendicular.length_squared()).abs().sqrt() * normal;
    r_perpendicular + r_parallel
}

fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0*r0;
    r0 + (1.0 - r0)*(1.0 - cosine).powf(5.0)
}