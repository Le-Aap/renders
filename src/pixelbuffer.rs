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
    pub fn iter(&self) -> PixelIterator<'_> {
        PixelIterator::new(self.colors.iter(), self.width)
    }

    /// Iterates over just the pixel locations, from left to right and top to bottom.
    #[must_use]
    pub fn iter_locations(&self) -> PixelLocationIterator {
        PixelLocationIterator::new(self.width, self.height)
    }
    
    /// Iterates over pixel left to right and then top to bottom.
    #[must_use]
    pub fn iter_mut(&mut self) -> PixelIteratorMut<'_> {
        PixelIteratorMut::new(self.colors.iter_mut(), self.width)
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

pub struct PixelLocationIterator {
    iter: core::ops::Range<usize>,
    width: usize,
}

impl PixelLocationIterator {
    #[must_use]
    fn new(width: usize, height: usize) -> Self {
        Self { iter: 0..(width * height), width }
    }
}

impl Iterator for PixelLocationIterator {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|i| {
            let y = i / self.width;
            let x = i % self.width;
            (x, y)
        })
    }
}

pub struct PixelIterator<'a> {
    iter: std::iter::Enumerate<std::slice::Iter<'a, Color>>,
    width: usize,
}

impl<'a> PixelIterator<'a> {
    fn new(iter: std::slice::Iter<'a, Color>, width: usize) -> Self {
        Self { iter: iter.enumerate(), width }
    }
}

impl<'a> Iterator for PixelIterator<'a> {
    type Item = (Color, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, color)| {
            let y = i / self.width;
            let x = i % self.width;
            (*color, x, y)
        })
    }
}

pub struct PixelIteratorMut<'a> {
    iter: std::iter::Enumerate<std::slice::IterMut<'a, Color>>,
    width: usize,
}

impl<'a> PixelIteratorMut<'a> {
    fn new(iter: std::slice::IterMut<'a, Color>, width: usize) -> Self {
        Self { iter: iter.enumerate(), width }
    }
}

impl<'a> Iterator for PixelIteratorMut<'a>{
    type Item = (&'a mut Color, usize, usize);
    
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(i, color)| {
            let y = i / self.width;
            let x = i % self.width;
            (color, x, y)
        })
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
        let mut buffer = PixelBuffer::new(5, 8);

        for (color, x, y) in buffer.iter_mut() {
            *color = Color::new((x as f64)/5.0, (y as f64)/8.0, 1.0);
        }

        for (color, x, y) in buffer.iter() {
            assert_eq!(color, Color::new((x as f64)/5.0, (y as f64)/8.0, 1.0));
        }

        for ((_, x, y),(xa, ya)) in buffer.iter().zip(buffer.iter_locations()) {
            assert_eq!(x, xa);
            assert_eq!(y, ya);
        }
        assert_eq!(buffer.iter().count(), buffer.iter_locations().count())
    }
}
