use colors::Color;
use ray_math::Ray;
use vec_math::{dot, Vec3};
use core::f64;
use std::{vec::Vec};

pub mod vec_math;
pub mod colors;
pub mod ray_math;

/// Calculates the color at the end of a ray.
/// If a bad color value is produced, black is returned instead.
#[must_use]
pub fn ray_color<T: Hittable>(ray: &Ray, world: &T) -> Color {
    match world.hit(ray, 0.0, f64::INFINITY) {
        Some(hit) => {
            ((hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5).try_into().unwrap_or_else(|_|{Color::new(0.0, 0.0, 0.0)})
        },
        None => {
            let a = 0.5 * (ray.direction().normalized().y() + 1.0);
            ((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)).try_into().unwrap_or_else(|_|{Color::new(0.0, 0.0, 0.0)})
        },
    }
}

#[allow(clippy::suspicious_operation_groupings)]
#[must_use]
pub fn sphere_intersection(center: &Vec3, radius: f64, ray: &Ray) -> Option<f64>{
    let oc = *center - *ray.origin();
    let a = ray.direction().square_length();
    let h = dot(ray.direction(), &oc);
    let c = oc.square_length() - radius * radius;
    let discriminant = h*h - a*c;

    if discriminant < 0.0 {
        None
    } else {
        Some((h - discriminant.sqrt()) / a)
    }
}

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

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
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
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
        if root <= tmin || tmax <= root {
            root = (h + discriminant) / a;
            if root <= tmin || tmax <= root {
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

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self { Self {objects: Vec::new()} }

    pub fn push<T>(&mut self, object: T)
    where 
        T: Hittable + 'static,
    {
        self.objects.push(Box::new(object));
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord> {
        let mut current = None;

        for hittable in &self.objects {
            if let Some(hit) = hittable.hit(ray, tmin, tmax) {
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
