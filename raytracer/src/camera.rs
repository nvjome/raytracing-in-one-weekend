use std::f64::INFINITY;
use crate::{color::{ppm_preamble, ppm_write_pixel, ColorRGB}, hittable::{HitRecord, Hittable}, interval::Interval, point::Point3, ray::Ray3, vector::Vector3};

#[derive(Default)]
pub struct Camera {
    position: Point3,
    pub image_width: i32,
    image_height: i32,
    pub aspect_ratio: f64,
    pixel_origin: Point3,
    pixel_delta_u: Vector3,
    pixel_delta_v: Vector3,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64) -> Camera {
        Camera {
            image_width,
            aspect_ratio,
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
                let pixel_center = self.pixel_origin + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray = Ray3::new(self.position, pixel_center - self.position);
                ppm_write_pixel(self.ray_color(ray, &world));
            }
        }

        eprintln!("\rDone                      ");
    }

    fn initialize(&mut self) {
        // Image
        self.image_height = (((self.image_width as f64) / self.aspect_ratio) as i32).max(1);

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

    fn ray_color(&self, ray: Ray3, world: &impl Hittable) -> ColorRGB {
        let mut record = HitRecord::new();
    
        if world.hit(ray, Interval::new(0.0, INFINITY), &mut record) {
            return 0.5 * record.normal + ColorRGB::new(0.5, 0.5, 0.5);
        }
    
        let unit_dir = ray.direction.unit();
        let a = 0.5 * (unit_dir.y + 1.0);
        return (1.0 - a) * ColorRGB::new(1.0, 1.0, 1.0) + a * ColorRGB::new(0.5, 0.7, 1.0);
    }
}