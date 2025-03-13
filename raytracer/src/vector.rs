use glam::DVec3;
use rand::{self, Rng};

// Supporting functions to work with vectors

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

pub fn random_unit_vector() -> DVec3 {
    return random_in_unit_sphere().normalize();
}

pub fn random_unit_on_hemisphere(normal: &DVec3) -> DVec3 {
    let unit_vec = random_unit_vector();
    if unit_vec.dot(*normal) > 0.0 {
        return unit_vec;
    } else {
        return -unit_vec;
    }
}