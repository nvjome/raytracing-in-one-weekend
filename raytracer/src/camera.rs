use std::f64::INFINITY;
use rand::{rngs::ThreadRng, Rng};
use crate::{
    color::{ppm_preamble, ppm_write_pixel, ColorRGB},
    hittable::{HitRecord, Hittable},
    point::Point3,
    ray::Ray3,
    vector::{self, Vector3}
};

#[derive(Default)]
pub struct Camera {
    position: Point3,
    pub image_width: i32,
    image_height: i32,
    pub aspect_ratio: f64,
    pixel_origin: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
    pub samples_per_pixel: i32,
    pixel_sample_scale: f64,
    pub max_depth: i32,
    rng: ThreadRng,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32, max_depth: i32) -> Camera {
        Camera {
            image_width,
            aspect_ratio,
            samples_per_pixel,
            max_depth,
            ..Default::default()
        }
    }

    pub fn render(&mut self, world: impl Hittable) {
        self.initialize();

        // PBM preamble
        ppm_preamble(self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {}   ", self.image_height - j);
            
            for i in 0..self.image_width {
                let (mut r, mut g, mut b) = (0.0, 0.0, 0.0);

                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(i, j);
                    let temp_color = self.ray_color(ray, self.max_depth, &world);
                    r += temp_color.r;
                    g += temp_color.g;
                    b += temp_color.b;
                }
                ppm_write_pixel(ColorRGB::new(
                    r / self.samples_per_pixel as f64,
                    g / self.samples_per_pixel as f64,
                    b / self.samples_per_pixel as f64
                ));
            }
        }

        eprintln!("\rDone                      ");
    }

    fn initialize(&mut self) {
        // Image
        self.image_height = (((self.image_width as f64) / self.aspect_ratio) as i32).max(1);
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        // Viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64) / (self.image_height as f64);

        // Camera geometry
        let focal_length = 1.0;
        self.position = Point3::new(0.0, 0.0, 0.0);

        let viewport_u = Vector3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vector3::new(0.0, -viewport_height, 0.0);
        
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_origin = self.position - Vector3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_origin = viewport_origin + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&mut self, ray: Ray3, depth: i32, world: &impl Hittable) -> ColorRGB {
        let mut record = HitRecord::new();
    
        if world.hit(ray, 0.001..INFINITY, &mut record) {
            let direction = vector::random_unit_on_hemisphere(&mut self.rng, record.normal);
            
            return 0.5 * self.ray_color(Ray3::new(record.p, direction), depth - 1, world);
        }
    
        let unit_dir = ray.direction.unit();
        let a = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - a) * ColorRGB::new(1.0, 1.0, 1.0) + a * ColorRGB::new(0.5, 0.7, 1.0);
    }

    fn get_ray(&mut self, i: i32, j: i32) -> Ray3 {
        let offset = self.sample_square();
        let pixel_sample = self.pixel_origin
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        
        return Ray3::new(self.position, pixel_sample - self.position);
    }

    fn sample_square(&mut self) -> Vector3 {
        // rng.random() returns [0.0, 1.0] for f64
        let x: f64 = self.rng.random();
        let y: f64 = self.rng.random();
        let z: f64 = self.rng.random();
        // Final result should be within [-0.5, 0.5] in all dimensions
        return Vector3::new(x - 0.5, y - 0.5, z - 0.5);
    }
}