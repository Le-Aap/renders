use colors::Color;
use ray_math::Ray;
use vec_math::{dot, Vec3};
use std::{vec::Vec};
use interval::Interval;
use camera::{Camera, CameraBuilder};

pub mod interval;
pub mod vec_math;
pub mod colors;
pub mod ray_math;
pub mod camera;

/// Calculates the color at the end of a ray.
/// If a bad color value is produced, black is returned instead.
#[must_use]
pub fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    world.hit(ray, &Interval::new(0.0, f64::INFINITY)).map_or_else(
    || {
        let a = 0.5 * (ray.direction().normalized().y() + 1.0);
        ((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0))
            .try_into()
            .unwrap_or_else(|_|{Color::new(0.0, 0.0, 0.0)})
    },
    |hit| ((hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5)
            .try_into()
            .unwrap_or_else(|_|{Color::new(0.0, 0.0, 0.0)}))
}

/// Type returned by all hits.
pub struct HitRecord {
    /// Point where the ray hit the hittable.
    pub point: Vec3,
    /// Surface normal at the hit point.
    pub normal: Vec3,
    /// Distance* travelled by the ray from the camera to the surface.
    pub t: f64,
    /// True if the surface hit is a front-face.
    pub front_face: bool,
}

/// Trait to be implemented for all things that can be hit by a ray.
pub trait Hittable {
    /// Intersects the ray with the surface and returns the hit if there was one.
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord>;
}

/// Represents a sphere with a surface. 
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    /// Creates a new sphere.
    /// # Panics
    /// panics if radius is set to be smaller than 0.
    #[must_use]
    pub fn new(center: Vec3, radius: f64) -> Self {
        assert!(radius >= 0.0);
        Self {center, radius}
    }
}

/// Calculates the normal and front facing value of a hit.
/// # Panics
/// Panics if `outward_normal` is not of unit length.
#[must_use]
pub fn calculate_face_normal(ray: &Ray, outward_normal: &Vec3) -> (bool, Vec3) {
    assert!((outward_normal.square_length() - 1.0).abs() <= 0.0001);

    let front_face = dot(ray.direction(), outward_normal) < 0.0;
    let normal = if front_face {*outward_normal} else {-1.0 * *outward_normal};
    (front_face, normal)
}

impl Hittable for Sphere {
    #[allow(clippy::suspicious_operation_groupings)]
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let oc = self.center - *ray.origin();
        let a = ray.direction().square_length();
        let h = dot(ray.direction(), &oc);
        let c = oc.square_length() - self.radius * self.radius;

        let discriminant = h*h - a*c;
        if discriminant < 0.0 {
            return None;
        }

        let discriminant = discriminant.sqrt();

        let mut root = (h - discriminant) / a;
        if !ray_t.surrounds(root) {
            root = (h + discriminant) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let hit_point = ray.at(root);
        let (front_face, normal) = calculate_face_normal(ray, &((hit_point - self.center) / self.radius));

        Some(HitRecord {
            t: root,
            point: hit_point,
            normal, front_face
        })
    }
}

/// A hittable collection of hittable items.
pub struct Hittables {
    objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    #[must_use]
    pub fn new() -> Self { Self {objects: Vec::new()} }

    /// Adds a hittable item into the collection
    pub fn add<T>(&mut self, object: T)
    where 
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(object));
    }

    /// Clears the collection of hittable items
    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

/// Defaults to an empty collection
impl Default for Hittables {
    /// returns an empty collection
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for Hittables {
    /// Returns the nearest hit to any object in the collection
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        let mut current = None;

        for hittable in &self.objects {
            if let Some(hit) = hittable.hit(ray, ray_t) {
                current = match current {
                    None => Some(hit),
                    Some(current_hit) => {
                        if (current_hit.t) > hit.t { 
                            Some(hit)
                        } else {
                            Some(current_hit)
                        }
                    }
                }
            }
        }

        current
    }
}
