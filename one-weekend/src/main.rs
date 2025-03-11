use raytracer::{
    hittable::HittableList,
    point::Point3,
    sphere::Sphere,
    camera::Camera,
};

fn main() {
    // Output
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 600;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut camera = Camera::new(image_width, aspect_ratio);

    // Render... It's a simple as that
    camera.render(world);
}