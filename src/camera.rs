use crate::{Hittable, colors::Color, interval::Interval, ray_math::Ray, vec_math::Vec3, pixelbuffer::PixelBuffer};
use rand;
use core::time;
use std::{
    fs::File, io::{BufWriter, prelude::*}, sync::{Arc, Mutex}, thread
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
/// - `max_bounces`: 10,
/// - `nr_threads`: 1,
#[derive(Debug, PartialEq)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    center: Vec3,
    focal_length: f64,
    viewport_height: f64,
    samples_per_pixel: u32,
    max_bounces: u32,
    nr_threads: usize
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
            nr_threads: 1,
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
            nr_threads: self.nr_threads,
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
            nr_threads: self.nr_threads,
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
            nr_threads: self.nr_threads,
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
            nr_threads: self.nr_threads,
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
            nr_threads: self.nr_threads,
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
            nr_threads: self.nr_threads,
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
            nr_threads: self.nr_threads,
        }
    }

    #[must_use]
    pub const fn set_nr_threads(self, nr_threads: usize) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            center: self.center,
            focal_length: self.focal_length,
            viewport_height: self.viewport_height,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads,
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
            nr_threads: self.nr_threads,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a view into a world.
/// Use `render` function to render the view as a ppm image.
/// A camera can be built using the `camerabuilder` struct.
/// # Example
/// ```
/// # use renders::{brdfs::{self, BRDF}, camera::CameraBuilder, colors::Color, vec_math::Vec3, Hittables, Sphere};
/// let mut world = Hittables::new();
///
/// let ground_material: BRDF = brdfs::make_lambertian_diffuse_brdf(Color::new(0.8, 0.8, 0.0));
/// let center_material: BRDF  = brdfs::make_lambertian_diffuse_brdf(Color::new(0.1, 0.2, 0.5));
///  
/// world.add(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0, ground_material));
/// world.add(Sphere::new(Vec3::new(0.0, 0.0, -1.2), 0.5, center_material));
///  
/// let camera = CameraBuilder::new()
///     .set_aspect_ratio(16.0/9.0)
///     .set_image_width(400)
///     .set_samples_per_pixel(300)
///     .set_nr_threads(8)
///     .to_camera();
/// 
/// // camera.render(&std::sync::Arc::new(world)); // call to render this view into the world.
/// ```
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
    nr_threads: usize,
}

impl Camera {
    /// Renders the cameras perspective in the world.
    /// # Panics
    /// Funcion may panic if any of the file opperations fail or if any of the render threads panic.
    pub fn render<T: Hittable + Send + Sync + 'static>(&self, world: &Arc<T>) {
        let output = Arc::new(Mutex::new((PixelBuffer::new(
            self.image_width.try_into().expect(
                "Creating pixel buffer failed: image wider than can be represented by usize",
            ),
            self.image_height.try_into().expect(
                "Creating pixel buffer failed: image higher than can be represented by usize",
            ),
        ), 0)));

        // Defining this function as a closure so that no reference to self ends up in a worker thread, rust does not allow that.
        let get_ray = {
            let pixel_origin = self.pixel_origin;
            let pixel_delta_u = self.pixel_delta_u;
            let pixel_delta_v = self.pixel_delta_v;
            let center = self.center;
            
            move |x: u32, y: u32| {
                let offset = sample_square();
                let pixel_sample = pixel_origin
                + ((f64::from(x) + offset.x()) * pixel_delta_u)
                + ((f64::from(y) + offset.y()) * pixel_delta_v);
                
                let ray_direction = pixel_sample - center;
                Ray::new(center, ray_direction)
            }
        };

        let mut render_threads = Vec::new();

        for id in 0..self.nr_threads {
            // Copying these values here so that no reference to self ends up in the render_thread closure as rust will not allow sending a closure with a reference to self accross threads.
            let nr_threads = self.nr_threads;
            let samples_per_pixel = self.samples_per_pixel;
            let max_bounces = self.max_bounces;
            let pixel_samples_scale = self.pixel_samples_scale;
            let world = world.clone();
            let pixel_iter = {output.lock().expect("Unable to get lock on mutex!").0.iter().filter(move |(_,y)| {(*y + id).is_multiple_of(nr_threads)})};
            let pixels = output.clone();

            let render_thread = move || {
                for (x, y) in pixel_iter {
                    let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                    for _ in 0..samples_per_pixel {
                        let camera_ray = get_ray(x.try_into().expect("Unable to cast usize to u32."), y.try_into().expect("Unable to cast usize to u32."));
                        pixel_color += ray_color(&camera_ray, max_bounces, world.as_ref()) * pixel_samples_scale;
                    }
                
                    pixel_color = pixel_color.to_gamma();
                
                    let mut out = pixels.lock().expect("Unable to get lock on mutex!");
                    out.0.set_pixel(x, y, pixel_color);
                    out.1 += 1;
                }
            };

            render_threads.push(thread::spawn(render_thread));
        }
        
        loop {
            if render_threads.iter().all(|thread| {thread.is_finished()}) {
                break;
            }
            let progress = {output.lock().expect("Unable to get lock on mutex").1};
            let progress: u32 = (progress as f64 / (self.image_height * self.image_width) as f64 * 100.0) as u32;
            print!("\rRendering ({progress}%)        ");
            thread::sleep(time::Duration::from_secs_f32(0.01));
        }

        for thread in render_threads {
            _ = thread.join();
        }

        print!("\rWriting to file      ");
        let mut file = BufWriter::new(File::create("image.ppm").expect("Error creating file."));
        file.write_all(output.lock().expect("Unable to get lock on mutex!").0.to_string().as_bytes())
            .expect("Error while writing to file buffer.");
        file.flush().expect("Error while flushing file buffer.");
        println!("\rDone                 ");
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rand::random::<f64>() - 0.5,
        rand::random::<f64>() - 0.5,
        0.0,
    )
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
            |hit| match (hit.brdf)(ray, &hit) {
                Some(reflection) => {
                    reflection.attenuation * ray_color(&reflection.reflected, depth - 1, world)
                }
                None => Color::new(0.0, 0.0, 0.0),
            },
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
