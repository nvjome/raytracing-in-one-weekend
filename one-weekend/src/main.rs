use std::{fs, io, sync::Arc, time::Instant};
use glam::DVec3;
use itertools::{self, Itertools};
use raytracer::{
    camera::CameraBuilder,
    hittable::HittableList,
    material::Material,
    sphere::Sphere
};

type Point3 = DVec3;
type Color = DVec3;

fn main() -> io::Result<()> {
    let now = Instant::now();
    // Output
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 800;
    let image_height = (((image_width as f64) / aspect_ratio) as i32).max(1);
    let samples_per_pixel = 100;
    let max_depth = 100;

    let output = "output/focus1.ppm";

    // Materials
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    };

    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    };

    let material_left = Material::Dielectric {
        refraction_index: 1.5,
    };

    let material_bubble = Material::Dielectric {
        refraction_index: 1.0 / 1.5,
    };

    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.5,
    };

    // World
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right
    )));

    // Camera
    let camera = CameraBuilder::new()
        .image(image_width, image_height)
        .pixel(samples_per_pixel, max_depth)
        .fov(20.0)
        .position(
            Point3::new(-2.0, 2.0, 1.0),
            Point3::new(0.0, 0.0, -1.0)
        )
        .up(DVec3::new(0.0, 1.0, 0.0))
        .focus(3.4, 10.0)
        .build();

    // Render image
    println!("Rendering...");
    let image = camera.render(Arc::new(world));

    // PPM preamble
    let preamble = format!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);

    // PPM pixel data
    let data = image.iter().map(|(r, g, b)| {
        format!("{} {} {}", r, g, b)
    })
    .join("\n");

    fs::write(output, format!("{}\n{}", preamble, data))?;

    let elapsed = now.elapsed();
    println!("Finished render in {:.2?}", elapsed);
    Ok(())
}