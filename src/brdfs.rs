use std::sync::Arc;
use crate::{HitRecord, Ray, colors::Color, vec_math::{Vec3, dot, reflect, refract, unit_vector}};

/// Represents the effects of a reflections: A reflected ray and some amount of light attenuation.
pub struct Reflection {
    pub reflected: Ray,
    pub attenuation: Color,
}

/// Type of a shader or as the technical term goes, a BRDF.
pub type BRDF = Arc<dyn Fn(Ray, &HitRecord) -> Option<Reflection> + Send + Sync>; 

/// For creating materials with the lambertian diffuse lighting model, for use with perfectly diffuse objects.
#[must_use]
pub fn make_lambertian_diffuse_brdf(albedo: Color) -> BRDF {
    let brdf = move |_incoming: Ray, hit: &HitRecord| {
        // Dont send out a ray if the ray is fully absorbed.
        if albedo == Color::new(0.0, 0.0, 0.0) {
            return None;
        }

        let mut scatter_direction = hit.normal + Vec3::random_unit_vector();
        // Handle edge case where scatter direction is almost zero which may cause floating point precision issues.
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let reflected = Ray::new(hit.point, scatter_direction);
        
        let attenuation = albedo;
        Some(
            Reflection {
                reflected,
                attenuation,
            }
        )
    };
    Arc::new(brdf)
}

/// For creating materials with the reflection characteristics of a metal.
#[must_use]
pub fn make_metal_brdf(albedo: Color) -> BRDF {
    let brdf = move |incoming: Ray, hit: &HitRecord| {
        if albedo == Color::new(0.0, 0.0, 0.0) {
            return None;
        }
        
        let reflection = reflect(incoming.direction(), hit.normal);
        let attenuation = albedo;
        let reflected = Ray::new(hit.point, reflection);
        Some(
            Reflection { reflected, attenuation }
        )
    };
    Arc::new(brdf)
}

/// For creating glass like materials
#[must_use]
pub fn make_glass_brdf(ior: f64, albedo: Color) -> BRDF {
    let brdf = move |incoming: Ray, hit: &HitRecord| {
        let refraction_constant = if hit.front_face {1.0/ior} else {ior};
        
        let unit_direction = unit_vector(incoming.direction());
        let cos_theta = dot(-unit_direction, hit.normal).min(1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta*cos_theta);

        let cannot_refract = refraction_constant * sin_theta > 1.0;
        let direction = if cannot_refract || reflectance(cos_theta, refraction_constant) > rand::random::<f64>() {
            reflect(unit_direction, hit.normal)
        } else {
            refract(unit_direction, hit.normal, refraction_constant)
        };
        
        let scattered = Ray::new(hit.point, direction);
        Some(
            Reflection { reflected: scattered, attenuation: albedo }
        )
    };
    Arc::new(brdf)
}

const fn reflectance(cosine: f64, ior: f64) -> f64 {
    let r0 = (1.0 - ior) / (1.0 + ior);
    (r0 * r0) + (1.0-(r0 * r0)) * const_pow5(1.0 - cosine)
}

const fn const_pow5(a: f64) -> f64 {
    a * a * a * a * a
}