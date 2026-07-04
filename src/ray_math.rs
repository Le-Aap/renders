use crate::vec_math::{Vec3, dot};

/// Struct for representing mathmatical rays, consisting of an origin and a direction.
/// # Example
/// ```
/// use renders::{ray_math::*, vec_math::*};
/// let origin = Vec3::new(0.0, 1.0, 0.0);
/// let direction = Vec3::new(0.0, -2.0, 0.0);
/// 
/// let ray = Ray::new(origin, direction);
/// assert_eq!(ray.direction(), direction);
/// assert_eq!(ray.origin(), origin);
/// 
/// assert_eq!(ray.at(1.0), Vec3::new(0.0, -1.0, 0.0));
/// ```
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    /// Creates a new ray with origin `origin` and direction `direction.normalized()`.
    #[must_use]
    pub const fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
        }
    }

    /// Returns the point t distance along the ray.
    /// ```
    /// use renders::{ray_math::*, vec_math::*};
    /// let example = Ray::new(
    /// Vec3::new(0.0, 1.0, 0.0),
    /// Vec3::new(1.0, 0.0, 0.0)
    /// );
    /// assert_eq!(example.at(4.0), Vec3::new(4.0, 1.0, 0.0));
    /// ```
    #[must_use]
    pub fn at(self, t:f64) -> Vec3 {
        self.origin() + self.direction() * t
    }

    /// Returns an immutable borrow from the origin.
    #[must_use]
    pub const fn origin(self) -> Vec3 {
        self.origin
    }

    /// Returns an immutable borrow from the direction.
    #[must_use]
    pub const fn direction(self) -> Vec3 {
        self.direction
    }

    /// Returns the value t for the distance at which this ray intersects the plane
    /// defined by `normal_x * x + normal_y * y + normal_z * z + height = 0`. Returns
    /// none if the ray and plane are parallel
    #[must_use]
    pub fn intersect_plane(self, normal: Vec3, height: f64) -> Option<f64> {
        let ndotr = dot(normal, self.direction);
        if ndotr.abs() < f64::EPSILON {
            None
        } else {
            Some(-(dot(normal, self.origin) + height) / ndotr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_ray_functionality() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(2.0, 0.0, 0.0);

        let ray = Ray::new(origin, direction);
        let point = ray.at(-2.0);
        assert_eq!(point, Vec3::new(-4.0, 0.0, 0.0));
        assert_eq!(origin, ray.origin());
        assert_eq!(direction, ray.direction());
    }

    
}
