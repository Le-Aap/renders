use std::{fmt::Debug, ops::Add};
use super::vec_math::Vec3;

// Todo: Color should support addition and subtraction with other color, and multiplication with f64s (like vectors).
// Todo: Colors should be able to be converted into Vec3s and the other way around (try into as not all vec3s are legal colors).

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Color {
    vector: Vec3,
}

#[derive(Debug)]
pub enum ColorError {
    RGBValOutOfRange(),
}

impl Color {
    /// Tries to create a color.
    /// # Errors
    /// If r, g, or b is not in range [0, 1] the function returns an `RGBValOutOfRange` Error.
    pub fn try_new(r:f64, g:f64, b:f64) -> Result<Self, ColorError> {
        if r > 1.0 || r < 0.0 {
            return Err(ColorError::RGBValOutOfRange());
        }
        if g > 1.0 || g < 0.0 {
            return Err(ColorError::RGBValOutOfRange());
        }
        if b > 1.0 || b < 0.0 {
            return Err(ColorError::RGBValOutOfRange());
        }

        let vector = Vec3::new(r, g, b);

        Ok(Self{vector})
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

mod tests {
    use super::*;

    #[test]
    fn vec3_and_color_conversion() {
        let color = Color::try_new(1.0, 0.5, 0.0).expect("This is impossible");
        let vec3:Vec3 = color.into();
        assert_eq!(vec3, Vec3::new(1.0, 0.5, 0.0));
    
        let vec3 = Vec3::new(1.0, 0.5, 0.0);
        let color2: Color = vec3.try_into().expect("This is impossible");
        assert_eq!(color2, color);
    }
    
    #[test]
    fn color_print_test() {
        let color = Color::try_new(1.0, 0.5, 0.0).expect("This is impossible");
        assert_eq!(color.to_string(), String::from("255 127 0\n"));
    }
}