use crate::vec_math::Vec3;

/// Struct for representing mathmatical rays, consisting of an origin and a direction.
/// # Example
/// ```
/// use renders::{ray_math::*, vec_math::*};
/// let origin = Vec3::new(0.0, 1.0, 0.0);
/// let direction = Vec3::new(0.0, -2.0, 0.0);
/// 
/// let ray = Ray::new(origin, direction);
/// assert_ne!(*ray.direction(), direction);
/// assert_eq!(*ray.direction(), Vec3::new(0.0, -1.0, 0.0)); // Ray direction is always of unit length.
/// 
/// assert_eq!(ray.at(1.0), Vec3::new(0.0, 0.0, 0.0));
/// ```
#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    /// Creates a new ray with origin `origin` and direction `direction.normalized()`.
    #[must_use]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
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
    pub fn at(&self, t:f64) -> Vec3 {
        *self.origin() + *self.direction() * t
    }

    /// Returns an immutable borrow from the origin.
    #[must_use]
    pub const fn origin(&self) -> &Vec3 {
        &self.origin
    }

    /// Returns an immutable borrow from the direction.
    #[must_use]
    pub const fn direction(&self) -> &Vec3 {
        &self.direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_creation() {
        let origin = Vec3::new(0.0, 0.0, 0.0);
        let direction = Vec3::new(2.0, 0.0, 0.0);

        let ray = Ray::new(origin, direction);
        let point = ray.at(-2.0);
        assert_eq!(point, Vec3::new(-2.0, 0.0, 0.0));
        assert_eq!(origin, *ray.origin());
        assert_eq!(direction.normalized(), *ray.direction());
    }
}