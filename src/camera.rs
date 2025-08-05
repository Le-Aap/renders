use crate::{Hittable, colors::Color, interval::Interval, ray_math::Ray, vec_math::Vec3};
use rand;
use std::{
    fs::File,
    io::{BufWriter, prelude::*},
};

/// Struct used to build a camera.
///
/// # Example
/// ```
/// # use renders::{camera::{CameraBuilder, Camera},vec_math::Vec3};
/// let camera = CameraBuilder::new()
///     .set_image_width(400)
///     .set_camera_center(Vec3::new(1.0, 0.0, -1.0))
///     .set_aspect_ratio(16.0 / 9.0)
///     .to_camera();
///
/// println!("{camera:#?}");
/// ```
/// ## Default values:
/// if a different value is not set with one of the functions the following defaults are used:
/// - `aspect_ratio`: 1.0,
/// - `image_width`: 100,
/// - `center`: (0.0, 0.0, 0.0),  (Vec3)
/// - `focal_length`: 1.0,
/// - `viewport_height`: 2.0,
/// - `samples_per_pixel`: 10,
/// - `max_bounces`: 10
#[derive(Debug, PartialEq)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    center: Vec3,
    focal_length: f64,
    viewport_height: f64,
    samples_per_pixel: u32,
    max_bounces: u32,
}

impl CameraBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            center: Vec3::new(0.0, 0.0, 0.0),
            focal_length: 1.0,
            viewport_height: 2.0,
            samples_per_pixel: 10,
            max_bounces: 10,
        }
    }

    #[must_use]
    pub const fn set_aspect_ratio(self, aspect_ratio: f64) -> Self {
        Self {
            aspect_ratio,
            image_width: self.image_width,
            center: self.center,
            focal_length: self.focal_length,
            viewport_height: self.viewport_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
        }
    }

    #[must_use]
    pub const fn set_image_width(self, image_width: u32) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width,
            center: self.center,
            focal_length: self.focal_length,
            viewport_height: self.viewport_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
        }
    }

    #[must_use]
    pub const fn set_camera_center(self, center: Vec3) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            center,
            focal_length: self.focal_length,
            viewport_height: self.viewport_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
        }
    }

    #[must_use]
    pub const fn set_focal_length(self, focal_length: f64) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            center: self.center,
            focal_length,
            viewport_height: self.viewport_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
        }
    }

    #[must_use]
    pub const fn set_viewport_height(self, viewport_height: f64) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            center: self.center,
            focal_length: self.focal_length,
            viewport_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
        }
    }

    #[must_use]
    pub const fn set_samples_per_pixel(self, samples_per_pixel: u32) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            center: self.center,
            focal_length: self.focal_length,
            viewport_height: self.viewport_height,
            samples_per_pixel,
            max_bounces: self.max_bounces,
        }
    }

    #[must_use]
    pub const fn set_max_bounces(self, max_bounces: u32) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            center: self.center,
            focal_length: self.focal_length,
            viewport_height: self.viewport_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces,
        }
    }

    #[must_use]
    pub fn to_camera(self) -> Camera {
        let image_width = self.image_width;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let image_height = ((f64::from(self.image_width) / self.aspect_ratio) as u32).max(1);

        let viewport_height = self.viewport_height;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / f64::from(image_width);
        let pixel_delta_v = viewport_v / f64::from(image_height);

        let viewport_upper_left = self.center
            - Vec3::new(0.0, 0.0, self.focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel_origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let pixel_samples_scale = 1.0 / f64::from(self.samples_per_pixel);

        Camera {
            image_width,
            image_height,
            center: self.center,
            pixel_origin,
            pixel_delta_u,
            pixel_delta_v,
            pixel_samples_scale,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Vec3,
    pixel_origin: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f64,
    samples_per_pixel: u32,
    max_bounces: u32,
}

impl Camera {
    /// Renders the cameras perspective in the world.
    /// # Panics
    /// Funcion may panic if any of the file opperations fail.
    pub fn render<T: Hittable>(&self, world: &T) {
        let mut file = BufWriter::new(File::create("image.ppm").expect("Error creating file."));

        file.write_all(
            format!("P3\n{0} {1}\n255\n", self.image_width, self.image_height).as_bytes(),
        )
        .expect("Error while writing to file-buffer");

        for j in 0..self.image_height {
            print!("\rScanlines remaining: {}        ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                
                for _ in 0..self.samples_per_pixel {
                    let camera_ray = self.get_ray(i, j);
                    pixel_color += ray_color(&camera_ray, self.max_bounces, world) * self.pixel_samples_scale;
                }

                pixel_color = pixel_color.to_gamma();

                file.write_all(pixel_color.to_string().as_bytes())
                    .expect("Error while writing to file-buffer");
            }
        }

        file.flush().expect("Error while executing file-writes");
        println!("\rDone.                           ");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_origin
            + ((f64::from(i) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(j) + offset.y()) * self.pixel_delta_v);
        
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(rand::random::<f64>() - 0.5, rand::random::<f64>() - 0.5, 0.0)
}

fn ray_color<T: Hittable>(ray: &Ray, depth: u32, world: &T) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    world
        .hit(ray, &Interval::new(0.00001, f64::INFINITY))
        .map_or_else(
            || {
                let a = 0.5 * (ray.direction().normalized().y() + 1.0);
                ((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)).into()
            },
            |hit| {
                match (hit.brdf)(ray, &hit) {
                    Some(reflection) => {
                        reflection.attenuation * ray_color(&reflection.reflected, depth - 1, world)
                    }
                    None => {
                        Color::new(0.0, 0.0, 0.0)
                    }
                }
                
            }
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_camera_creation() {
        let camera_a = CameraBuilder::new()
            .set_aspect_ratio(1.0)
            .set_camera_center(Vec3::new(0.0, 0.0, 0.0))
            .set_focal_length(1.0)
            .set_image_width(100)
            .set_viewport_height(2.0)
            .to_camera();

        let camera_b = CameraBuilder::new()
            .set_viewport_height(2.0)
            .set_camera_center(Vec3::new(0.0, 0.0, 0.0))
            .set_aspect_ratio(1.0)
            .set_image_width(100)
            .set_focal_length(1.0)
            .to_camera();

        assert_eq!(camera_a, CameraBuilder::default().to_camera());
        assert_eq!(camera_b, camera_a);

        let camera_c = CameraBuilder::new()
            .set_aspect_ratio(2.0)
            .set_camera_center(Vec3::new(3.0, 0.0, 4.0))
            .set_focal_length(1.5)
            .set_image_width(2_000_000)
            .set_viewport_height(12.0)
            .to_camera();

        assert_ne!(camera_a, camera_c);
    }
}
