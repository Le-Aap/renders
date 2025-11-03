use std::fmt::Display;
use crate::Color;

/// A structure that provides a 2d interface to write pixel values.
pub struct PixelBuffer {
    colors: Vec<Color>,
    width: usize,
    height: usize,
}

impl PixelBuffer {
    /// Sets up a new color buffer with the bounds provided. All pixels are initialized to black.
    #[must_use]
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let initial_color = Color::new(0.0, 0.0, 0.0);
        let colors = vec![initial_color;size];

        Self {
            colors,
            width,
            height,
        }
    }

    /// Set pixel at coordinate x, y. Both x and y are zero indexed
    /// # Panics
    /// Panics if x or y fail a bounds check
    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        assert!(x < self.width);
        assert!(y < self.height);
        self.colors[y * self.width + x] = color;
    }

    /// Gets pixel at coordinate x, y. Both x and y are zero indexed.
    /// # Panics
    /// Panics if x is outside of the range `[0, width)`.
    /// Panics if y is outside of the range `[0, height)`.
    #[must_use]
    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        assert!(x < self.width);
        assert!(y < self.height);
        self.colors[y * self.width + x]
    }

    /// Iterates over pixels left to right and then top to bottom.
    #[must_use]
    pub fn iter(&self) -> PixelIterator {
        <&Self as IntoIterator>::into_iter(self)
    }
}

/// Displays pixel buffer as a ppm image
impl Display for PixelBuffer {
    /// Prints the pixel data stored in the pixel buffer as a .ppm image
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "P3\n{0} {1}\n255\n{2}",
            self.width,
            self.height,
            self.colors
                .iter()
                .map(|color| { color.to_string() })
                .collect::<String>()
        )
    }
}

impl IntoIterator for &PixelBuffer {
    type Item = (usize, usize);

    type IntoIter = PixelIterator;

    /// Make an iterator over all pixel values.
    fn into_iter(self) -> Self::IntoIter {
        PixelIterator::new(self.width, self.height)
    }
}

/// Iterator over the indices in a pixel buffer.
/// # Example
/// ```
/// # use renders::pixelbuffer::*;
/// let pixels = PixelBuffer::new(5, 5);
/// 
/// let mut pixel_iter = pixels.iter();
/// assert_eq!(pixel_iter.next(), Some((0, 0)));
/// assert_eq!(pixel_iter.next(), Some((1, 0)));
/// assert_eq!(pixel_iter.next(), Some((2, 0)));
/// assert_eq!(pixel_iter.next(), Some((3, 0)));
/// assert_eq!(pixel_iter.next(), Some((4, 0)));
/// assert_eq!(pixel_iter.next(), Some((0, 1)));
/// // etc.
/// ```
pub struct PixelIterator {
    current: usize,
    width: usize,
    max: usize,
}

impl Iterator for PixelIterator {
    type Item = (usize, usize);

    /// Traverse pixels row by row.
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let curr: usize = self.current;
            let width: usize = self.width;

            self.current += 1;

            Some((curr % width, curr / width))
        } else {
            None
        }
    }
}

impl PixelIterator {
    const fn new(width: usize, height: usize) -> Self {
        Self {
            current: 0,
            max: width * height,
            width,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic_usage() {
        let mut buffer = PixelBuffer::new(10, 10);
        assert_eq!(buffer.get_pixel(4, 4), Color::new(0.0, 0.0, 0.0));
        buffer.set_pixel(5, 6, Color::new(1.0, 1.0, 1.0));
        assert_eq!(buffer.get_pixel(4, 4), Color::new(0.0, 0.0, 0.0));
        assert_eq!(buffer.get_pixel(5, 6), Color::new(1.0, 1.0, 1.0));
    }

    #[test]
    fn iteration() {
        let mut buffer = PixelBuffer::new(5, 5);
        for (x, y) in &buffer {
            assert_eq!(buffer.get_pixel(x, y), Color::new(0.0, 0.0, 0.0));
        }

        for (x, y) in &buffer {
            buffer.set_pixel(x, y, Color::new(0.5, 0.5, 0.5));
        }

        for (x, y) in &buffer {
            assert_ne!(buffer.get_pixel(x, y), Color::new(0.0, 0.0, 0.0));
        }
    }
}