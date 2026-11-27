use ggmath::{Element, PrimitiveFloat, Vec3};

pub trait Sdf<T>
where
    T: Element,
{
    /// Evaluate the signed distance function at point p.
    fn eval(&self, p: Vec3<T>) -> T;

    /// Returns SDF equal to the union of `self` and `other`.
    fn union<U>(self, other: U) -> Union<Self, U>
    where
        Self: Sized,
        U: Sdf<T>,
    {
        Union { a: self, b: other }
    }

    /// Returns SDF equal to the intersection of `self` and `other`.
    fn intersect<I>(self, other: I) -> Intersect<Self, I>
    where
        Self: Sized,
        I: Sdf<T>,
    {
        Intersect { a: self, b: other }
    }

    /// Returns SDF equal to the compliment of `self`.
    fn compliment(self) -> Compliment<Self>
    where
        Self: Sized,
    {
        Compliment { a: self }
    }

    /// Returns SDF equal to subtracting `other` from `self`.
    /// Equivalent to `self.intersect(other.compliment)`
    fn subtract<O>(self, other: O) -> Intersect<Self, Compliment<O>>
    where
        Self: Sized,
        O: Sdf<T>,
    {
        Intersect {
            a: self,
            b: other.compliment(),
        }
    }

    /// Returns SDF equal to `self` translated by `t`.
    fn translate_by(self, t: Vec3<T>) -> Translated<Self, T>
    where
        Self: Sized,
    {
        Translated { s: self, t }
    }

    /// Returns SDF equal to `self` scaled uniformally by factor `t`.
    fn scale_by(self, t: T) -> Scaled<Self, T>
    where
        Self: Sized,
    {
        Scaled { s: self, t }
    }
}

/// SDF of the unit sphere.
#[derive(Debug, Clone, PartialEq, Eq)]
struct UnitSphere {}

impl<T> Sdf<T> for UnitSphere
where
    T: PrimitiveFloat,
{
    fn eval(&self, p: Vec3<T>) -> T {
        Vec3::length(p) - T::ONE
    }
}

/// SDF of a sphere in 3D space, radius must be positive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SphereSdf<T>
where
    T: Element,
{
    s: Translated<Scaled<UnitSphere, T>, T>,
}

impl<T> SphereSdf<T>
where
    T: PrimitiveFloat,
{
    /// Creates a new SDF of a sphere centered at `center` in 3D space with radius >= 0 `radius`.
    /// # Panics
    /// Panics when trying to create a sphere with negative radius.
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        assert!(radius >= T::ZERO);
        Self {
            s: UnitSphere {}.scale_by(radius).translate_by(center),
        }
    }
}

impl<T> Sdf<T> for SphereSdf<T>
where
    T: PrimitiveFloat,
{
    fn eval(&self, p: Vec3<T>) -> T {
        self.s.eval(p)
    }
}

/// Union combinator type. See Sdf trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Union<A, B> {
    a: A,
    b: B,
}

impl<A, B, T> Sdf<T> for Union<A, B>
where
    T: PrimitiveFloat,
    A: Sdf<T>,
    B: Sdf<T>,
{
    fn eval(&self, p: Vec3<T>) -> T {
        let da = self.a.eval(p);
        let db = self.b.eval(p);

        if da < db { da } else { db }
    }
}

/// Intersection combinator type. See Sdf trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Intersect<A, B> {
    a: A,
    b: B,
}

impl<A, B, T> Sdf<T> for Intersect<A, B>
where
    T: PrimitiveFloat,
    A: Sdf<T>,
    B: Sdf<T>,
{
    fn eval(&self, p: Vec3<T>) -> T {
        let da = self.a.eval(p);
        let db = self.b.eval(p);

        if da > db { da } else { db }
    }
}

/// Compliment combinator type. See Sdf trait.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Compliment<A> {
    a: A,
}

impl<A, T> Sdf<T> for Compliment<A>
where
    T: PrimitiveFloat,
    A: Sdf<T>,
{
    fn eval(&self, p: Vec3<T>) -> T {
        -self.a.eval(p)
    }
}

/// Combinator type for translations of SDFs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Translated<S, T: Element> {
    s: S,
    t: Vec3<T>,
}

impl<S, T> Sdf<T> for Translated<S, T>
where
    T: PrimitiveFloat,
    S: Sdf<T>,
{
    fn eval(&self, p: Vec3<T>) -> T {
        self.s.eval(p - self.t)
    }
}

/// Combinator type for uniformly scaling SDFs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scaled<S, T> {
    s: S,
    t: T,
}

impl<S, T> Sdf<T> for Scaled<S, T>
where
    T: PrimitiveFloat,
    S: Sdf<T>,
{
    fn eval(&self, p: Vec3<T>) -> T {
        self.s.eval(p / self.t) * self.t
    }
}

/// Sdf of a box centered at the origin aligned with the axes.
#[derive(Debug, Clone, PartialEq, Eq)]
struct CenteredBoxSdf<T>
where
    T: Element,
{
    b: Vec3<T>,
}

impl<T> CenteredBoxSdf<T>
where
    T: Element,
{
    const fn new(b: Vec3<T>) -> Self {
        Self { b }
    }
}

impl<T> Sdf<T> for CenteredBoxSdf<T>
where
    T: PrimitiveFloat,
{
    fn eval(&self, p: Vec3<T>) -> T {
        let q = p.abs() - self.b;
        let maxelement = q.max_element();
        Vec3::length(q.max(Vec3::ZERO))
            + if maxelement < T::ZERO {
                T::ZERO
            } else {
                maxelement
            }
    }
}

/// Sdf of a box aligned with the axes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoxSdf<T>
where
    T: Element,
{
    b: Translated<CenteredBoxSdf<T>, T>,
}

impl<T> BoxSdf<T>
where
    T: PrimitiveFloat,
{
    pub fn new(center: Vec3<T>, radius: Vec3<T>) -> Self {
        Self {
            b: CenteredBoxSdf::new(radius).translate_by(center),
        }
    }

    #[allow(clippy::suspicious_operation_groupings)]
    pub fn from_bounds(lower: Vec3<T>, upper: Vec3<T>) -> Self {
        // Clipy thinks this code is bad too (:
        Self::new(
            (lower + upper) / (T::ONE + T::ONE),
            (upper - lower) / (T::ONE + T::ONE),
        )
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    static UNIT_SPHERE: LazyLock<SphereSdf<f32>> =
        LazyLock::new(|| SphereSdf::new(Vec3::ZERO, 1.0_f32));

    #[test]
    fn sphere_sdf() {
        let sphere = UNIT_SPHERE.clone();
        assert!(f32::abs(sphere.eval(Vec3::ZERO) + 1.0) <= f32::EPSILON);
        dbg!(&sphere);
        assert_eq!(sphere, *UNIT_SPHERE);
    }

    #[test]
    fn union_sdf() {
        let center2 = Vec3::X;
        let center3 = Vec3::Y;

        let sphere = UNIT_SPHERE.clone();
        let sphere2 = SphereSdf::new(center2, 1.0);
        let sphere3 = SphereSdf::new(center3, 1.0);

        let combined = sphere.union(sphere2).union(sphere3);
        assert!(f32::abs(combined.eval(Vec3::ZERO) + 1.0) <= f32::EPSILON);
        assert!(f32::abs(combined.eval(center2) + 1.0) <= f32::EPSILON);
        assert!(f32::abs(combined.eval(center3) + 1.0) <= f32::EPSILON);
    }

    #[test]
    fn intersection_sdf() {
        let sphere = UNIT_SPHERE.clone();
        let sphere2 = SphereSdf::new(Vec3::X, 1.0);

        let intersection = sphere.intersect(sphere2);
        assert!(f32::abs(intersection.eval(Vec3::ZERO)) <= f32::EPSILON);
    }

    #[test]
    fn compliment_sdf() {
        let sphere = UNIT_SPHERE.clone();

        let compliment = sphere.compliment();
        assert!(f32::abs(compliment.eval(Vec3::ZERO) - 1.0) <= f32::EPSILON);
    }

    #[test]
    fn subtract_sdf() {
        let sphere = UNIT_SPHERE.clone();
        let sphere2 = SphereSdf::new(Vec3::X, 1.0);

        let subtracted = sphere.subtract(sphere2);
        assert!(f32::abs(subtracted.eval(Vec3::ZERO)) <= f32::EPSILON);
        assert!(f32::abs(subtracted.eval(-Vec3::X)) <= f32::EPSILON);
    }

    #[test]
    fn translate_sdf() {
        let sphere = UNIT_SPHERE.clone();
        let translated = sphere.translate_by(Vec3::Z);
        let sphere2 = SphereSdf::new(Vec3::Z, 1.0);
        assert!(
            f32::abs(translated.eval(Vec3::X * 2.0) - sphere2.eval(Vec3::X * 2.0)) <= f32::EPSILON
        );
    }

    #[test]
    fn scale_sdf() {
        let sphere = UNIT_SPHERE.clone();
        let scaled = sphere.scale_by(2.0);
        let sphere2 = SphereSdf::new(Vec3::ZERO, 2.0);
        assert!(f32::abs(scaled.eval(Vec3::X * 4.0) - sphere2.eval(Vec3::X * 4.0)) <= f32::EPSILON);
    }

    #[test]
    fn box_sdf() {
        let boxsdf = BoxSdf::new(Vec3::Z * 2.0, Vec3::new(1.0, 1.0, 2.0));
        let boxsdf2 = BoxSdf::from_bounds(Vec3::new(-1.0, -1.0, 0.0), Vec3::new(1.0, 1.0, 4.0));
        assert_eq!(boxsdf, boxsdf2);
    }
}
