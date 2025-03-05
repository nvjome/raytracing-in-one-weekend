use raytracer::{
    ray::Ray3,
    vector::Vector3,
    point::Point3,
    color::{ColorRGB, ppm_preamble, ppm_write_pixel},
};

fn hit_sphere(center: &Point3, radius: f64, ray: Ray3) -> bool {
    let vect_oc = ray.origin.get_vector_to(center);
    let a = ray.direction.dot(ray.direction);
    let b = -2.0 * ray.direction.dot(vect_oc);
    let c = vect_oc.dot(vect_oc) - radius*radius;
    let discriminant = b*b - 4.0*a*c;
    discriminant >= 0.0
}

fn ray_color(ray: &Ray3) -> ColorRGB {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, *ray) {
        return ColorRGB::new(1.0, 0.0, 0.0)
    }

    let unit_dir = ray.direction.unit();
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * ColorRGB::new(1.0, 1.0, 1.0) + a * ColorRGB::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (((image_width as f64) / aspect_ratio) as i32).max(1);

    // Viewport
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

    // Camera
    let focal_length = 1.0;
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
    
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    let viewport_origin: Point3 = camera_center + Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_origin: Point3 = viewport_origin + 0.5 * (pixel_delta_u + pixel_delta_v);

    // PBM preamble
    ppm_preamble(image_width, image_height);

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {}   ", image_height - j);
        for i in 0..image_width {
            let pixel_center: Point3 = pixel_origin + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_dir = camera_center.get_vector_to(&pixel_center);
            let ray = Ray3::new(camera_center, ray_dir);
            ppm_write_pixel(ray_color(&ray));
        }
    }

    eprintln!("\rDone                      ");
}
