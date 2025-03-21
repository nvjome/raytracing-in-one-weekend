use std::sync::Arc;
use rayon::prelude::*;
use glam::DVec3;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use crate::{
    hittable::Hittable,
    ray::Ray3,
    vector_utils,
};

#[derive(Default, Copy, Clone)]
pub struct Camera {
    pub image_width: i32,
    pub image_height: i32,
    pixel_origin: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub location: DVec3,
    defocus_disk_u: DVec3,
    defocus_disk_v: DVec3,
}

impl Camera {
    pub fn render(self, world: Arc<dyn Hittable>) -> Vec<(u32, u32, u32)> {
        let bar = ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        bar.set_style(ProgressStyle::default_bar());
        // Generate iterator for all pixels
        (0..self.image_height * self.image_width)
            .into_par_iter()
            .map(|index| {
                // Extract (x, y) from iterator
                let x = index % self.image_width;
                let y = index / self.image_width;

                let world = Arc::clone(&world);
                let scale_factor = (self.samples_per_pixel as f64).recip();
                
                let multisampled_pixel_color = (0..self.samples_per_pixel)
                    .map(|_| {
                        // Get a ray, then get the color of that ray
                        self.get_ray(x, y).color(self.max_depth, &*world)
                    })
                    // Sum all samples and scale
                    .sum::<DVec3>() * scale_factor;

                // Scale and clamp
                let color = DVec3 {
                    x: linear_to_gamma(multisampled_pixel_color.x),
                    y: linear_to_gamma(multisampled_pixel_color.y),
                    z: linear_to_gamma(multisampled_pixel_color.z),
                }.clamp(DVec3::splat(0.0), DVec3::splat(0.999)) * 256.0;

                bar.inc(1);

                // Color tuple
                (color.x as u32, color.y as u32, color.z as u32)
            }).collect::<Vec<(u32, u32, u32)>>()
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray3 {
        let offset = sample_square();
        let pixel_sample = self.pixel_origin
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let origin = match self.defocus_disk_u.length() <= 0.0 {
            true => self.location,
            false => self.sample_defocus_disk(),
        };
        let direction = pixel_sample - origin;
        
        Ray3::new(origin, direction)
    }

    fn sample_defocus_disk(&self) -> DVec3 {
        let offset = vector_utils::random_in_unit_disk();
        self.location + (offset.x * self.defocus_disk_u) + (offset.y * self.defocus_disk_v)
    }
}

pub struct CameraBuilder {
    image_width: i32,
    image_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    vertical_fov: f64,
    relative_up: DVec3,
    position: DVec3,
    point_at: DVec3,
    focus_distance: f64,
    defocus_angle: f64,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl CameraBuilder {
    pub fn new() -> Self {
        // Some reasonable default settings
        CameraBuilder {
            image_width: 800,
            image_height: 400,
            samples_per_pixel: 10,
            max_depth: 10,
            vertical_fov: 90.0,
            relative_up: DVec3 { x: 0.0, y: 1.0, z: 0.0 },
            position: DVec3 { x: 0.0, y: 0.0, z: 0.0 },
            point_at: DVec3 { x: 0.0, y: 0.0, z: -1.0 },
            focus_distance: 1.0,
            defocus_angle: 0.0,
        }
    }

    pub fn build(self) -> Camera {
        // Image settings
        let image_width = self.image_width.max(1);
        let image_height = self.image_height.max(1);
        let samples_per_pixel = self.samples_per_pixel.max(1);
        let max_depth = self.max_depth.max(1);

        // Viewport dimensions
        let theta = self.vertical_fov.clamp(0.0, 180.0).to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

        // Camera geometry
        let w = (self.position - self.point_at).normalize();
        let u = self.relative_up.cross(w).normalize();
        let v = w.cross(u);

        // Viewport and pixel grid geometry
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_origin = self.position - (self.focus_distance * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel_origin = viewport_origin + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Defocus
        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;
        
        Camera {
            image_width,
            image_height,
            pixel_origin,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
            location: self.position,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    // Modifier functions to edit camera parameters prior to building
    pub fn image(mut self, width: i32, height: i32) -> Self {
        self.image_width = width;
        self.image_height = height;
        self
    }

    pub fn pixel(mut self, samples: i32, depth: i32) -> Self {
        self.samples_per_pixel = samples;
        self.max_depth = depth;
        self
    }

    pub fn fov(mut self, vertical_fov: f64) -> Self {
        self.vertical_fov = vertical_fov;
        self
    }

    pub fn position(mut self, camera_location: DVec3, point_at: DVec3) -> Self {
        self.position = camera_location;
        self.point_at = point_at;
        self
    }

    pub fn up(mut self, direction: DVec3) -> CameraBuilder {
        self.relative_up = direction;
        self
    }

    pub fn focus(mut self, distance: f64, defocus: f64) -> CameraBuilder {
        self.focus_distance = distance;
        // Scale defocus value from range [0, 1] to angle [0, 180]
        self.defocus_angle = defocus.clamp(0.0, 180.0);
        self
    }
}

fn sample_square() -> DVec3 {
    let mut rng = rand::rng();
    // rng.random() returns [0.0, 1.0] for f64
    let x: f64 = rng.random();
    let y: f64 = rng.random();
    let z: f64 = rng.random();
    // Final result should be within [-0.5, 0.5] in all dimensions
    DVec3::new(x - 0.5, y - 0.5, z - 0.5)
}

fn linear_to_gamma(scalar: f64) -> f64 {
    if scalar > 0.0 {
        scalar.sqrt()
    } else {
        0.0
    }
}