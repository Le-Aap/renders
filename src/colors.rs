use std::{fmt::Debug, ops::{Add, Div, Mul, Sub}};
use super::vec_math::Vec3;

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    vector: Vec3,
}

#[derive(Debug)]
pub enum ColorError {
    RGBValOutOfRange(),
}

impl Color {
    /// Creates a new Color with values RGB in the range [0,1].
    /// # Example:
    /// ```
    /// use renders::colors::Color;
    /// let _ = Color::new(0.0, 0.5, 1.0);
    /// ```
    pub fn new(r:f64, g:f64, b:f64) -> Self {
        if r > 1.0 || r < 0.0 {
            panic!("RGB value should be in range [0,1]")
        }
        if g > 1.0 || g < 0.0 {
            panic!("RGB value should be in range [0,1]")
        }
        if b > 1.0 || b < 0.0 {
            panic!("RGB value should be in range [0,1]")
        }

        let vector = Vec3::new(r, g, b);

        Self{vector}
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    #[allow(clippy::cast_sign_loss)]
    pub fn to_string(&self) -> String {
        let r = self.vector.x * 255.999;
        let g = self.vector.y * 255.999;
        let b = self.vector.z * 255.999;

        let r = r.floor() as u8;
        let g = g.floor() as u8;
        let b = b.floor() as u8;
        format!("{r} {g} {b}\n")
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let result = self.vector + rhs.vector;
        result.try_into().expect("Error! RGB values must be in range [0,1]")
    }
}

impl Mul<f64> for Color {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        let result = self.vector * rhs;
        result.try_into().expect("Error! RGB values must be in range [0,1]")
    }
}

impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        let result = rhs.vector * self;
        result.try_into().expect("Error! RGB values must be in range [0,1]")
    }
}

impl Div<f64> for Color {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        let result = self.vector / rhs;
        result.try_into().expect("Error! RGB values must be in range [0,1]")
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let result = self.vector - rhs.vector;
        result.try_into().expect("Error! RGB values must be in range [0,1]")
    }
}

impl TryFrom<Vec3> for Color {
    type Error = ColorError;
    fn try_from(vector: Vec3) -> Result<Self, ColorError> {
        if vector.x > 1.0 || vector.x < 0.0 {
            return Err(ColorError::RGBValOutOfRange());
        }
        if vector.y > 1.0 || vector.y < 0.0 {
            return Err(ColorError::RGBValOutOfRange());
        }
        if vector.z > 1.0 || vector.z < 0.0 {
            return Err(ColorError::RGBValOutOfRange());
        }
        Ok(Self{vector})
    }
}

impl From<Color> for Vec3 {
    fn from(value: Color) -> Self {
        value.vector
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
        let color2: Color = vec3.try_into().expect("This is impossible");
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
    #[should_panic]
    fn bounds_checking_add() {
        let _ = Color::new(1.0, 0.5, 0.0) + Color::new(1.0, 0.5, 0.0);
    }

    #[test]
    #[should_panic]
    fn bounds_checking_sub() {
        let _ = Color::new(1.0, 0.5, 0.0) - Color::new(2.0, 0.5, 0.0);
    }

    #[test]
    #[should_panic]
    fn bounds_checking_mul() {
        let _ = Color::new(1.0, 0.5, 0.0) * 10.0;
    }

    #[test]
    #[should_panic]
    fn bounds_checking_div() {
        let _ = Color::new(1.0, 0.5, 0.0) / 0.1;
    }
}