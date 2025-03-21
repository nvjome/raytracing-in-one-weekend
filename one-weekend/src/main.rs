use std::{fs, io, sync::Arc, time::Instant};
use glam::DVec3;
use itertools::{self, Itertools};
use rand::Rng;
use raytracer::{
    camera::CameraBuilder,
    hittable::HittableList,
    material::Material,
    sphere::Sphere, vector_utils
};

type Point3 = DVec3;
type Color = DVec3;

fn main() -> io::Result<()> {
    let now = Instant::now();
    // Output
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1920;
    let image_height = (((image_width as f64) / aspect_ratio) as i32).max(1);
    let samples_per_pixel = 500;
    let max_depth = 100;

    let output = "output/final_scene.ppm";

    let mut world = HittableList::new();

    /*
    let material_ground = Material::Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0)
    };

    let material_center = Material::Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5)
    };

    let material_left = Material::Dielectric {
        refraction_index: 1.5
    };

    let material_bubble = Material::Dielectric {
        refraction_index: 1.0 / 1.5
    };

    let material_right = Material::Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.5
    };

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0), 100.0, material_ground
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2), 0.5, material_center
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0), 0.5, material_left
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0), 0.5, material_right
    )));

    let camera = CameraBuilder::new()
        .image(image_width, image_height)
        .pixel(samples_per_pixel, max_depth)
        .fov(20.0)
        .position(
            Point3::new(-2.0, 2.0, 1.0),
            Point3::new(0.0, 0.0, -1.0)
        )
        .up(DVec3::new(0.0, 1.0, 0.0))
        .focus(3.4, 5.0)
        .build();
    */

    // Ground
    let ground = Material::Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5)
    };
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0), 1000.0, ground)
    ));

    let mut rng = rand::rng();

    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.random();
            let center = Point3::new(a as f64 + 0.8 * rng.random::<f64>(), 0.2, b as f64 + 0.8 * rng.random::<f64>());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let new_sphere = if choose_material < 0.8 {
                    // Lambertian
                    let albedo = vector_utils::random_unit_vector() * vector_utils::random_unit_vector();
                    Sphere::new(center, 0.2, Material::Lambertian { albedo })
                } else if choose_material > 0.95 {
                    // Metal
                    let albedo = Color::new(
                        rng.random_range(0.5..=1.0),
                        rng.random_range(0.5..=1.0),
                        rng.random_range(0.5..=1.0)
                    );
                    let fuzz = rng.random_range(0.0..=0.5);
                    Sphere::new(center, 0.2, Material::Metal { albedo, fuzz })
                } else {
                    // Glass
                    Sphere::new(center, 0.2, Material::Dielectric { refraction_index: 1.5 })
                };
            
            world.add(Box::new(new_sphere));
            }
        }
    }

    let material1 = Material::Dielectric { refraction_index: 1.5 };
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1
    )));

    let material2 = Material::Lambertian { albedo: Color::new(0.4, 0.2, 0.1) };
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2
    )));

    let material3 = Material::Metal { albedo: Color::new(0.7, 0.6, 0.5), fuzz: 0.0 };
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3
    )));

    let camera = CameraBuilder::new()
        .image(image_width, image_height)
        .pixel(samples_per_pixel, max_depth)
        .fov(20.0)
        .position(
            Point3::new(13.0, 2.0, 3.0),
            Point3::new(0.0, 0.0, 0.0)
        )
        .up(DVec3::new(0.0, 1.0, 0.0))
        .focus(10.0, 0.6)
        .build();
    
    println!("Rendering...");
    let image = camera.render(Arc::new(world));

    let preamble = format!("P3\n{} {}\n255\n", camera.image_width, camera.image_height);

    let data = image.iter().map(|(r, g, b)| {
        format!("{} {} {}", r, g, b)
    })
    .join("\n");

    fs::write(output, format!("{}\n{}", preamble, data))?;

    let elapsed = now.elapsed();
    println!("Finished render in {:.2?}", elapsed);

    Ok(())
}