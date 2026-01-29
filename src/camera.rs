use crate::{Hittable, colors::Color, interval::Interval, pixelbuffer::PixelBuffer, ray_math::Ray, vec_math::{Vec3, cross, unit_vector}};
use rand;
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
///     .set_look_from(Vec3::new(1.0, 0.0, -1.0))
///     .set_aspect_ratio(16.0 / 9.0)
///     .to_camera();
///
/// println!("{camera:#?}");
/// ```
/// ## Default values:
/// if a different value is not set with one of the functions the following defaults are used:
/// - `aspect_ratio`: 1.0,
/// - `image_width`: 100,
/// - `camera_up`: (0.0, 1.0, 0.0),  (Vec3)
/// - `focal_length`: 1.0,
/// - `viewport_height`: 2.0,
/// - `samples_per_pixel`: 10,
/// - `max_bounces`: 10,
/// - `nr_threads`: 1,
/// - `vfov`: 50.0,
/// - `look_from`: (0.0, 0.0, 0.0)
/// - `look_at`: (0.0, 0.0, -1.0)
#[derive(Debug, PartialEq)]
pub struct CameraBuilder {
    aspect_ratio: f64,
    image_width: u32,
    camera_up: Vec3,
    focal_length: f64,
    samples_per_pixel: u32,
    max_bounces: u32,
    nr_threads: usize,
    look_from: Vec3,
    look_at: Vec3,
    vfov: f64,
}

impl CameraBuilder {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            camera_up: Vec3::new(0.0, 1.0, 0.0),
            focal_length: 1.0,
            samples_per_pixel: 10,
            max_bounces: 10,
            nr_threads: 1,
            vfov: 50.0,
            look_from: Vec3::new(0.0, 0.0, 0.0),
            look_at: Vec3::new(0.0, 0.0, -1.0),
        }
    }

    #[must_use]
    pub const fn set_aspect_ratio(self, aspect_ratio: f64) -> Self {
        Self {
            aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_image_width(self, image_width: u32) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_camera_up(self, vup: Vec3) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: vup,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_focal_length(self, focal_length: f64) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_samples_per_pixel(self, samples_per_pixel: u32) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_max_bounces(self, max_bounces: u32) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_nr_threads(self, nr_threads: usize) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_vfov(self, vfov: f64) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov,
            look_at: self.look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_look_at(self, look_at: Vec3) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at,
            look_from: self.look_from,
        }
    }

    #[must_use]
    pub const fn set_look_from(self, look_from: Vec3) -> Self {
        Self {
            aspect_ratio: self.aspect_ratio,
            image_width: self.image_width,
            camera_up: self.camera_up,
            focal_length: self.focal_length,
            samples_per_pixel: self.samples_per_pixel,
            max_bounces: self.max_bounces,
            nr_threads: self.nr_threads,
            vfov: self.vfov,
            look_at: self.look_at,
            look_from,
        }
    }

    #[must_use]
    pub fn to_camera(self) -> Camera {
        let image_width = self.image_width;
        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        let image_height = ((f64::from(self.image_width) / self.aspect_ratio) as u32).max(1);

        let pixel_samples_scale = 1.0 / f64::from(self.samples_per_pixel);
        
        let center = self.look_from;

        let focal_length = (self.look_at - self.look_from).length();
        let theta = f64::to_radians(self.vfov);
        let h = f64::tan(theta/2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * f64::from(image_width) / f64::from(image_height);

        let w = unit_vector(self.look_from - self.look_at);
        let u = unit_vector(cross(self.camera_up, w));
        let v = cross(w, u);

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta_u = viewport_u / f64::from(image_width);
        let pixel_delta_v = viewport_v / f64::from(image_height);

        let viewport_upper_left = center - (focal_length * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        let pixel_origin = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
        
        Camera {
            image_width,
            image_height,
            center,
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
    pub fn render<T>(&self, world: &T)
    where
        T: Send + Sync + Hittable
    {
        let output = Arc::new(Mutex::new(PixelBuffer::new(
            self.image_width.try_into().expect(
                "Creating pixel buffer failed: image wider than can be represented by usize",
            ),
            self.image_height.try_into().expect(
                "Creating pixel buffer failed: image higher than can be represented by usize",
            ),
        )));

        thread::scope(|s| {
            for id in 0..self.nr_threads {
                let output = output.clone();
                s.spawn(move || {
                    let pixel_iter = {
                        output.lock().expect("Lock should be available.")
                            .iter_locations().filter(|(_, y)| {
                                (y + id).is_multiple_of(self.nr_threads)
                            })
                    };
                    for (x, y) in pixel_iter {
                        let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                        for _ in 0..self.samples_per_pixel {
                            let camera_ray = self.get_ray(x.try_into().expect("An image with a width representable as a usize but not as a u32 is almost impossible."), y.try_into().expect("An image with height representable as a usize but not as a u32 is almost impossible."));
                            pixel_color += ray_color(camera_ray, self.max_bounces, world) * self.pixel_samples_scale;
                        }

                        pixel_color = pixel_color.to_gamma();

                        let mut out = output.lock().expect("This lock should be available in a reasonable time.");
                        out.set_pixel(x, y, pixel_color);
                    }
                });
            }
        });

        print!("\rWriting to file      ");
        let mut file = BufWriter::new(File::create("image.ppm").expect("We should be able to create this file."));
        file.write_all(output.lock().expect("We should be the only thread with access to this lock so this should always succeed.").to_string().as_bytes())
            .expect("We should be able to write to this file.");
        file.flush().expect("We should be able to write to this file.");
        println!("\rDone                 ");
    }
    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let offset = sample_square();
        let pixel_sample = self.pixel_origin
            + ((f64::from(x) + offset.x()) * self.pixel_delta_u)
            + ((f64::from(y) + offset.y()) * self.pixel_delta_v);
                    
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(
        rand::random::<f64>() - 0.5,
        rand::random::<f64>() - 0.5,
        0.0,
    )
}

fn ray_color<T: Hittable>(ray: Ray, depth: u32, world: &T) -> Color {
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    world
        .hit(ray, Interval::new(0.00001, f64::INFINITY))
        .map_or_else(
            || {
                let a = 0.5 * (ray.direction().normalized().y() + 1.0);
                ((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)).into()
            },
            |hit| (hit.brdf)(ray, &hit).map_or_else(
                || Color::new(0.0, 0.0, 0.0),
                |reflection| reflection.attenuation * ray_color(reflection.reflected, depth - 1, world)
            )
        )
}
