use glam::DVec3;
use rand::{self, Rng};

// Supporting functions to work with vectors

// Generate a random unit vector
pub fn random_unit_vector() -> DVec3 {
    random_in_unit_sphere().normalize()
}

// Generate a random vector in the unit sphere
pub fn random_in_unit_sphere() -> DVec3 {
    let mut rng = rand::rng();
    loop {
        let p = DVec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
        );

        let p_length = p.length_squared();

        if 1e-160 < p_length && p_length <= 1.0 {
            break p;
        }
    }
}

// Generatre a random unit vector on a disk in the x-y plane
pub fn random_in_unit_disk() -> DVec3 {
    let mut rng = rand::rng();
    loop {
        let p = DVec3::new(
            rng.random_range(-1.0..1.0),
            rng.random_range(-1.0..1.0),
            0.0,
        );

        let p_length = p.length_squared();

        if 1e-160 < p_length && p_length <= 1.0 {
            break p;
        }
    }
}

// Generate a random unit vector in the same hemisphere as another vector
pub fn random_in_unit_hemisphere(normal: &DVec3) -> DVec3 {
    let unit_vec = random_unit_vector();
    if unit_vec.dot(*normal) > 0.0 {
        unit_vec
    } else {
        -unit_vec
    }
}

// Check for very small vectors
pub fn near_zero(vector: DVec3) -> bool {{
    let s = 1e-8;
    vector.x < s && vector.y < s && vector.z < s
}}