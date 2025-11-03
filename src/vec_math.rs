use std::ops;

use crate::interval::Interval;

/// Struct for representing 3d Math vectors.
#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct Vec3 {
    x:f64,
    y:f64,
    z:f64,
}

impl Vec3 {
    #[must_use]
    pub const fn new(x:f64, y:f64, z:f64) -> Self {
        Self{x, y, z}
    }

    /// Returns the length of the vector
    /// # Example
    /// ```
    /// use renders::vec_math::Vec3;
    /// let example = Vec3::new(3.0, 4.0, 0.0);
    /// assert_eq!(example.length(), 5.0);
    /// ```
    #[must_use]
    pub fn length(&self) -> f64 {
        self.square_length().sqrt()
    }

    /// Returns the length of the vector squared. This is more performant than the regular length because it avoids an expensive square root.
    /// # Example
    /// ```
    /// use renders::vec_math::Vec3;
    /// let example = Vec3::new(3.0, 4.0, 0.0);
    /// assert_eq!(example.square_length(), 25.0);
    /// ```
    #[must_use]
    pub fn square_length(&self) -> f64 {
        self.z.mul_add(
            self.z,
            self.x.mul_add(
            self.x,
            self.y * self.y
            )
        )
    } 

    /// Returns a random vector with x, y and z in the range [0.0, 1.0]
    #[must_use]
    pub fn random() -> Self {
        Self { x: rand::random(), y: rand::random(), z: rand::random() }
    }

    /// Returns a random vector with x, y and z in the provided range.
    #[must_use]
    pub fn random_range(range: &Interval) -> Self {
        Self {
            x: (range.max() - range.min()).mul_add(rand::random::<f64>(), range.min()),
            y: (range.max() - range.min()).mul_add(rand::random::<f64>(), range.min()),
            z: (range.max() - range.min()).mul_add(rand::random::<f64>(), range.min()),
        }
    }

    /// Returns a random vector that lies on the unit sphere.
    #[must_use]
    pub fn random_unit_vector() -> Self {
        loop {
            let p = Self::random();
            let square_length = p.square_length();
            if 1e-160 < square_length && square_length <= 1.0 {
                return p / square_length.sqrt();
            }
        }
    }

    /// Returns a random vector that lies on the unit hemisphere that surrounds the normal vector.
    #[must_use]
    pub fn random_on_hemisphere(normal: &Self) -> Self{
        let on_unit_sphere = Self::random_unit_vector();
        if dot(&on_unit_sphere, normal) > 0.0 {
            on_unit_sphere
        } else {
            -1.0 * on_unit_sphere
        }
    }

    /// Returns an unit-vector with the same direction.
    /// # Example
    /// ```
    /// use renders::vec_math::*;
    /// let example = Vec3::new(5.0, 4.0, 3.0);
    /// let normalized = example.normalized();
    /// assert_eq!(normalized.length(), 1.0);
    /// ```
    #[must_use]
    pub fn normalized(&self) -> Self {
        *self / self.length()
    }

    /// Returns true if x y and z of the vector are very near to zero.
    #[must_use]
    pub fn near_zero(&self) -> bool {
        self.x.abs() < 1e-8 && self.y.abs() < 1e-8 && self.z.abs() < 1e-8
    }

    #[must_use]
    pub const fn x(&self) -> f64 {
        self.x
    }

    #[must_use]
    pub const fn y(&self) -> f64 {
        self.y
    }

    #[must_use]
    pub const fn z(&self) -> f64 {
        self.z
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::Mul<Self> for Vec3 {
    type Output = Self;

    /// Performs memberwise multiplication, for dot product use dot(a, b)
    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

/// Returns a unit vector with same direction as v. Identical to `v.normalized()`.
#[must_use]
pub fn unit_vector(v:&Vec3) -> Vec3 {
    v.normalized()
}

/// Returns the dot product of a and b.
#[must_use]
pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.x.mul_add(b.x,
    a.y.mul_add(b.y, 
    a.z * b.z
    ))
}

/// Returns the reflection of a, with normal vector n. NOTE: assumes n is normalized.
#[must_use]
pub fn reflect(a: &Vec3, n: &Vec3) -> Vec3 {
    *a - 2.0 * dot(a, n) * *n
}

/// Returns the cross product of a and b.
#[must_use]
pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3 {
        x: a.y.mul_add(b.z, -(a.z * b.y)),
        y: a.z.mul_add(b.x, -(a.x * b.z)),
        z: a.x.mul_add(b.y, -(a.y * b.x)),
    }
}

#[cfg(test)]
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;

    #[test]
    fn basic_creation() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(position, Vec3{x:1.0, y:2.0, z:3.0});
    }

    #[test]
    fn equals_opperator() {
        let position = Vec3::new(1.0, 2.0, 3.0);
        assert!(position != Vec3::new(1.0, 1.0, 3.0));
        assert!(position != Vec3::new(2.0, 2.0, 3.0));
        assert!(position != Vec3::new(1.0, 2.0, 2.0));
    }

    #[test]
    fn simple_opperations() {
        let mut position = Vec3::new(1.0, 2.0, 3.0);
        let position2 = Vec3::new(3.0, 2.0, 1.0);
    
        assert_eq!(position-position2, Vec3::new(-2.0, 0.0, 2.0));
        assert_eq!(position+position2, Vec3::new(4.0, 4.0, 4.0));
        assert_eq!(position * 2.0, Vec3::new(2.0, 4.0, 6.0));
        assert_eq!(position * 2.0, 2.0 * position);
        assert_eq!(position / 2.0, Vec3::new(0.5, 1.0, 1.5));
        
        position-=position2;
        assert_eq!(position, Vec3::new(-2.0, 0.0, 2.0));
    
        position+=position2;
        assert_eq!(position, Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn lengths() {
        let position = Vec3::new(2.0, 3.0, -1.0);
        let expected_squared = 14.0;
    
        assert_eq!(position.square_length(), expected_squared);
        assert_eq!(position.length(), expected_squared.sqrt());
    }

    #[test]
    fn normalization_test() {
        let a = Vec3::new(2.0, 3.0, -1.0);
    
        assert_eq!(a.normalized(), unit_vector(&a));
        assert_eq!(a.normalized(), a/a.length());
    }

    #[test]
    fn dot_test() {
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = Vec3::new(-1.0, 0.0, 0.0);
    
        assert_eq!(dot(&a, &b), 0.0);
        assert_eq!(dot(&a, &c), -1.0);
        assert_eq!(dot(&a, &a), 1.0);
    }

    #[test]
    fn cross_test() {
        let a = Vec3::new(2.0, 3.0, 4.0);
        let b = Vec3::new(5.0, 6.0, 7.0);
    
        assert_eq!(cross(&a, &b), Vec3::new(-3.0, 6.0, -3.0));
    }
}
