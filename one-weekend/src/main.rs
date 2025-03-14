use std::io;
use glam::DVec3;
use raytracer::{
    camera::Camera, hittable::HittableList, material::Material, sphere::Sphere
};

type Point3 = DVec3;
type Color = DVec3;

fn main() -> io::Result<()> {
    // Output
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;

    let output = "output/materials4.ppm";

    // Materials
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };

    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };

    let material_left = Material::Dielectric {
        refraction_index: 1.0 / 1.33,
    };

    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    // World
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right)));

    // Camera
    let mut camera = Camera::new(image_width, aspect_ratio, samples_per_pixel, max_depth);

    // Render... It's a simple as that
    camera.render(world, output)?;
    Ok(())
}