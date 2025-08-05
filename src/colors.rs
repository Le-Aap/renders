use std::{fmt::{Debug, Display}, ops::{Add, AddAssign, Div, Mul, Sub}};
use crate::interval::Interval;

use super::vec_math::Vec3;

/// Used to store an RGB value where R, G and B are in range \[0, 1\].
#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    vector: Vec3,
}

impl Color {
    /// Creates a new Color with values RGB in the range \[0,1\].
    /// Values outside of the range will get clamped.
    /// # Example:
    /// ```
    /// use renders::colors::Color;
    /// let _ = Color::new(0.0, 0.5, 1.0);
    /// ```
    #[must_use]
    pub fn new(r:f64, g:f64, b:f64) -> Self {
        let rgb_range = Interval::new(0.0, 1.0);
        let r = rgb_range.clamp(r);
        let g = rgb_range.clamp(g);
        let b = rgb_range.clamp(b);
        let vector = Vec3::new(r, g, b);
        Self{vector}
    }

    #[must_use]
    pub fn to_gamma(&self) -> Self  {
        Self {
            vector: Vec3::new(
                linear_to_gamma(self.vector.x()),
                linear_to_gamma(self.vector.y()),
                linear_to_gamma(self.vector.z()),
            )
        }
    }
}

impl Display for Color {
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = self.vector.x();
        let g = self.vector.y();
        let b = self.vector.z();

        let r = r * 255.999;
        let g = g * 255.999;
        let b = b * 255.999;

        let r = r.floor() as u8;
        let g = g.floor() as u8;
        let b = b.floor() as u8;

        writeln!(f, "{r} {g} {b}")
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let result = self.vector + rhs.vector;
        result.into()
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let result = self.vector * rhs;
        result.into()
    }
}

impl Mul<Self> for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self { vector: self.vector * rhs.vector }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        let result = rhs.vector * self;
        result.into()
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let result = self.vector / rhs;
        result.into()
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let result = self.vector - rhs.vector;
        result.into()
    }
}

impl From<Vec3> for Color {
    fn from(value: Vec3) -> Self {
        let rgb_range = Interval::new(0.0, 1.0);
        let r = rgb_range.clamp(value.x());
        let g = rgb_range.clamp(value.y());
        let b = rgb_range.clamp(value.z());
        Self { vector: Vec3::new(r, g, b) }
    }
}

impl From<Color> for Vec3 {
    fn from(value: Color) -> Self {
        value.vector
    }
}

fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    }
    else {
        0.0
    }
}

#[allow(clippy::should_panic_without_expect)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn vec3_and_color_conversion() {
        let color = Color::new(1.0, 0.5, 0.0);
        let vec3:Vec3 = color.into();
        assert_eq!(vec3, Vec3::new(1.0, 0.5, 0.0));
    
        let vec3 = Vec3::new(1.0, 0.5, 0.0);
        let color2: Color = vec3.into();
        assert_eq!(color2, color);
    }
    
    #[test]
    fn color_print_test() {
        let color = Color::new(1.0, 0.5, 0.0);
        assert_eq!(color.to_string(), String::from("255 127 0\n"));
    }

    #[test]
    fn color_arithmetic_test() {
        let color = Color::new(0.5, 0.5, 0.5);

        assert_eq!(color + color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(color * 2.0, Color::new(1.0, 1.0, 1.0));
        assert_eq!(2.0 * color, Color::new(1.0, 1.0, 1.0));
        assert_eq!(color / 0.5, Color::new(1.0, 1.0, 1.0));
        assert_eq!(color - color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_clamping_add() {
        assert_eq!(Color::new(1.0, 0.5, 0.0) + Color::new(1.0, 0.5, 0.0), Color::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn color_clamping_sub() {
        assert_eq!(Color::new(1.0, 0.5, 0.0) - Color::new(2.0, 0.5, 0.0), Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn color_clamping_mul() {
        assert_eq!(Color::new(1.0, 0.5, 0.0) * 10.0, Color::new(1.0, 1.0, 0.0));
    }

    #[test]
    fn color_clamping_div() {
        assert_eq!(Color::new(1.0, 0.5, 0.0) / 0.1, Color::new(1.0, 1.0, 0.0));
    }
}