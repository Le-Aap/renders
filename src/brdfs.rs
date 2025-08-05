use std::rc::Rc;
use crate::{colors::Color, vec_math::{reflect, Vec3}, HitRecord, Ray};

/// Represents the effects of a reflections: A reflected ray and some amount of light attenuation.
pub struct Reflection {
    pub reflected: Ray,
    pub attenuation: Color,
}

/// Type of a shader or as the technical term goes, a BRDF.
pub type BRDF = Rc<dyn Fn(&Ray, &HitRecord) -> Option<Reflection>>; 

/// For creating materials with the lambertian diffuse lighting model, for use with perfectly diffuse objects.
#[must_use]
pub fn make_lambertian_diffuse_brdf(albedo: Color) -> BRDF {
    let brdf = move |_incoming: &Ray, hit: &HitRecord| {
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
    Rc::new(brdf)
}

/// For creating materials with the reflection characteristics of a metal.
#[must_use]
pub fn make_metal_brdf(albedo: Color) -> BRDF {
    let brdf = move |incoming: &Ray, hit: &HitRecord| {
        if albedo == Color::new(0.0, 0.0, 0.0) {
            return None;
        }
        
        let reflection = reflect(incoming.direction(), &hit.normal);
        let attenuation = albedo;
        let reflected = Ray::new(hit.point, reflection);
        Some(
            Reflection { reflected, attenuation }
        )
    };
    Rc::new(brdf)
}