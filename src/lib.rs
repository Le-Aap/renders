use colors::Color;
use ray_math::Ray;
use vec_math::{dot, Vec3};

pub mod vec_math;
pub mod colors;
pub mod ray_math;

/// Calculates the color at the end of a ray.
/// # Panics
/// Panics if otherwise an invalid color would have been output.
#[must_use]
pub fn ray_color(ray: &Ray) -> Color {
    let center = Vec3::new(0.0, 0.0, -1.0);
    let radius = 0.5;

    if let Some(t) = sphere_intersection(&center, radius, ray) {
        let normal = (ray.at(t) - center).normalized();
        return (0.5 * Vec3::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0)).try_into().expect("Color out of RGB range!")
    }
       
    let a = 0.5 * (ray.direction().normalized().y() + 1.0);
    ((1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0)).try_into().expect("Color out of RGB range!")
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