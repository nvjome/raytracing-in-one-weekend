use std::f64::INFINITY;

use raytracer::{
    color::{ppm_preamble, ppm_write_pixel, ColorRGB},
    hittable::{HitRecord, Hittable, HittableList},
    interval::Interval,
    point::Point3,
    ray::Ray3,
    sphere::Sphere,
    vector::Vector3
};

fn ray_color(ray: Ray3, world: &impl Hittable) -> ColorRGB {
    let mut record = HitRecord::new();

    if world.hit(ray, Interval::new(0.0, INFINITY), &mut record) {
        return 0.5 * record.normal + ColorRGB::new(0.5, 0.5, 0.5);
    }

    let unit_dir = ray.direction.unit();
    let a = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - a) * ColorRGB::new(1.0, 1.0, 1.0) + a * ColorRGB::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;
    let image_height = (((image_width as f64) / aspect_ratio) as i32).max(1);

    // Viewport
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

    // Camera
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
    
    let pixel_delta_u: Vector3 = viewport_u / image_width as f64;
    let pixel_delta_v: Vector3 = viewport_v / image_height as f64;

    let viewport_origin: Point3 = camera_center - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_origin: Point3 = viewport_origin + 0.5 * (pixel_delta_u + pixel_delta_v);

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // PBM preamble
    ppm_preamble(image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}   ", image_height - j);
        for i in 0..image_width {
            let pixel_center = pixel_origin + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray = Ray3::new(camera_center, pixel_center - camera_center);
            ppm_write_pixel(ray_color(ray, &world));
        }
    }

    eprintln!("\rDone                      ");
}
