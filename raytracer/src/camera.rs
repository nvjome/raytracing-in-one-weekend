use std::sync::Arc;
use rayon::prelude::*;
use glam::DVec3;
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use crate::{
    hittable::Hittable,
    ray::Ray3,
};

#[derive(Default, Copy, Clone)]
pub struct Camera {
    position: DVec3,
    pub image_width: i32,
    pub image_height: i32,
    pub aspect_ratio: f64,
    pixel_origin: DVec3,
    pixel_delta_u: DVec3,
    pixel_delta_v: DVec3,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
}

impl Camera {
    pub fn new(image_width: i32, aspect_ratio: f64, samples_per_pixel: i32, max_depth: i32) -> Camera {
        let image_height = (((image_width as f64) / aspect_ratio) as i32).max(1);

        // Viewport
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (image_width as f64) / (image_height as f64);

        // Camera geometry
        let focal_length = 1.0;
        let position = DVec3::new(0.0, 0.0, 0.0);

        let viewport_u = DVec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = DVec3::new(0.0, -viewport_height, 0.0);
        
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        let viewport_origin = position - DVec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_origin = viewport_origin + 0.5 * (pixel_delta_u + pixel_delta_v);
        
        Camera {
            position,
            image_width,
            image_height,
            aspect_ratio,
            pixel_origin,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn render(self, world: Arc<dyn Hittable>) -> Vec<(u32, u32, u32)> {
        let bar = ProgressBar::new(self.image_height as u64 * self.image_width as u64);
        bar.set_style(ProgressStyle::default_bar());
        // Generate iterator for all pixels
        let pixels = (0..self.image_height * self.image_width)
            .into_par_iter()
            .map(|index| {
                // Extract (x, y) from iterator
                let x = index % self.image_width;
                let y = index / self.image_width;

                let world = Arc::clone(&world);
                let scale_factor = (self.samples_per_pixel as f64).recip();
                
                let multisampled_pixel_color = (0..self.samples_per_pixel)
                    .into_iter()
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
            }).collect::<Vec<(u32, u32, u32)>>();

        return pixels;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray3 {
        let offset = sample_square();
        let pixel_sample = self.pixel_origin
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);
        
        return Ray3::new(self.position, pixel_sample - self.position);
    }
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

fn linear_to_gamma(scalar: f64) -> f64 {
    if scalar > 0.0 {
        return scalar.sqrt();
    } else {
        return 0.0;
    }
}