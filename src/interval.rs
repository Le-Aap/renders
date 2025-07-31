#[derive(PartialEq, Debug)]
/// Defines a real interval.
pub struct Interval {
    min: f64,
    max: f64,
}

impl Interval {
    #[must_use]
    pub const fn new(min: f64, max: f64) -> Self { Self {min, max} } 

    /// Calculates the signed size of the interval.
    #[must_use]
    pub fn size(&self) -> f64 { self.max - self.min }

    /// Returns true if x is contained in the interval: min <= x <= max
    #[must_use]
    pub fn contains(&self, x: f64) -> bool { self.min <= x && x <= self.max }
    
    /// Returns true if x is surrounded by the interval: min < x < max
    #[must_use]
    pub fn surrounds(&self, x: f64) -> bool { self.min < x && x < self.max }

    /// Creates the empty interval that contains nothing.
    #[must_use]
    pub const fn empty() -> Self { Self { min: f64::INFINITY, max: -f64::INFINITY } }

    /// Creates the universe interval that contains everything.
    #[must_use]
    pub const fn universe() -> Self { Self { min: -f64::INFINITY, max: f64::INFINITY } }

    /// Returns the min value of the interval.
    #[must_use]
    pub const fn min(&self) -> f64 { self.min }

    /// Returns the max value of the interval.
    #[must_use]
    pub const fn max(&self) -> f64 { self.max }
}

impl Default for Interval {
    /// The default interval is the empty interval.
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn interval_creation() {
        assert_eq!(Interval::new(f64::INFINITY, -f64::INFINITY), Interval::empty());
        assert_eq!(Interval::new(-f64::INFINITY, f64::INFINITY), Interval::universe());
        assert_ne!(Interval::new(0.0, 1.0), Interval::new(1.0, 3.0));
        let default_interval: Interval = Interval::default();
        assert_eq!(default_interval, Interval::empty());
    }

    #[test]
    fn interval_size() {
        let a = Interval::new(0.0, 1.0);
        let b = Interval::new(1.0, 2.0);
        let c = Interval::new(-1.0, 1.0);

        assert!(a.size() - b.size() <= f64::EPSILON);
        assert!(a.size() - 1.0 <= f64::EPSILON);
        assert!(c.size() - 2.0 <= f64::EPSILON);
        assert!(c.size() - b.size() > f64::EPSILON);
    }

    #[test]
    fn interval_float_comparison() {
        assert!(!Interval::empty().contains(0.0));
        assert!(!Interval::empty().contains(f64::INFINITY));
        assert!(!Interval::empty().contains(-f64::INFINITY));
        assert!(!Interval::empty().contains(1_000_000.0));
        assert!(!Interval::empty().contains(-1_000_000.0));

        assert!(Interval::universe().contains(0.0));
        assert!(Interval::universe().contains(f64::INFINITY));
        assert!(Interval::universe().contains(-f64::INFINITY));
        assert!(Interval::universe().contains(1_000_000.0));
        assert!(Interval::universe().contains(-1_000_000.0));

        let a = Interval::new(-1.0, 1.0);
        assert!(a.contains(0.0));
        assert!(a.surrounds(0.0));
        assert!(a.contains(1.0));
        assert!(!a.surrounds(1.0));
        assert!(a.contains(-1.0));
        assert!(!a.surrounds(-1.0));

        assert!(!a.contains(-2.0));
        assert!(!a.surrounds(-2.0));
        assert!(!a.contains(2.0));
        assert!(!a.surrounds(2.0));
        assert!(!a.contains(f64::INFINITY));
        assert!(!a.surrounds(f64::INFINITY));
        assert!(!a.contains(-f64::INFINITY));
        assert!(!a.surrounds(-f64::INFINITY));
    }
}