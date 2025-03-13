use std::{self, fs, io};
use glam::DVec3;
use indicatif::ProgressIterator;
use itertools::{self, Itertools};
use rand::Rng;
use crate::{
    hittable::{HitRecord, Hittable},
    ray::Ray3,
    vector_utils
};

#[derive(Default)]
pub struct Camera {
    position: DVec3,
    pub image_width: i32,
    image_height: i32,
    pub aspect_ratio: f64,
    pixel_origin: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    pub samples_per_pixel: i32,
    pixel_sample_scale: f64,
    pub max_depth: i32,
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

    pub fn render(&mut self, world: impl Hittable, output_file: &str) -> io::Result<()> {
        self.initialize();

        // Generate iterator for all pixels, with progress bar
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .progress_count(self.image_height as u64 * self.image_width as u64)
            .map(|(y, x)| {
                // Given a particular (x, y) pixel, calculate the pixel's color
                let scale_factor = (self.samples_per_pixel as f64).recip();
                let sampled_pixel_color = (0..self.samples_per_pixel)
                    .into_iter()
                    .map(|_| {
                        let ray = self.get_ray(x, y);
                        ray_color(ray, self.max_depth, &world) * 255.0 * scale_factor
                    })
                    .sum::<DVec3>();

                // Format for PPM file
                format!(
                    "{} {} {}",
                    sampled_pixel_color.x as u32,
                    sampled_pixel_color.y as u32,
                    sampled_pixel_color.z as u32
                )
            })
            .join("\n");

        // Write out PPM preamble and pixel color values
        fs::write(output_file, format!("P3\n{} {}\n255\n{}", self.image_width, self.image_height, pixels))?;

        Ok(())
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
        self.position = DVec3::new(0.0, 0.0, 0.0);

        let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);
        
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        let viewport_origin = self.position - DVec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_origin = viewport_origin + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray3 {
        
        let offset = sample_square();
        let pixel_sample = self.pixel_origin
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        
        return Ray3::new(self.position, pixel_sample - self.position);
    }
}

fn ray_color(ray: Ray3, depth: i32, world: &impl Hittable) -> DVec3 {
    if depth <= 0 {
        return DVec3::new(0.0, 0.0, 0.0);
    }

    let mut record = HitRecord::new();

    if world.hit(ray, 0.001..f64::INFINITY, &mut record) {
        // Get normal vector from hit location
        let direction = vector_utils::random_unit_on_hemisphere(&record.normal);
        // Get color of next ray using recursion (for now)
        return 0.5 * ray_color(Ray3::new(record.p, direction), depth - 1, world);
        // return 0.5 * DVec3::new(record.normal.x + 1.0, record.normal.y + 1.0, record.normal.z + 1.0)
    }

    let unit_dir = ray.direction.normalize();
    let a = 0.5 * (unit_dir.y + 1.0);
    return (1.0 - a) * DVec3::new(1.0, 1.0, 1.0) + a * DVec3::new(0.5, 0.7, 1.0);
}

fn sample_square() -> DVec3 {
    let mut rng = rand::rng();
    // rng.random() returns [0.0, 1.0] for f64
    let x: f64 = rng.random();
    let y: f64 = rng.random();
    let z: f64 = rng.random();
    // Final result should be within [-0.5, 0.5] in all dimensions
    return DVec3::new(x - 0.5, y - 0.5, z - 0.5);
}