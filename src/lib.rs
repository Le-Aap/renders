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
}

/// SDF of a sphere in 3D space, radius must be positive.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sphere<T>
where
    T: Element,
{
    center: Vec3<T>,
    radius: T,
}

impl<T> Sphere<T>
where
    T: PrimitiveFloat,
{
    /// Creates a new SDF of a sphere centered at `center` in 3D space with radius >= 0 `radius`.
    /// # Panics
    /// Panics when trying to create a sphere with negative radius.
    pub fn new(center: Vec3<T>, radius: T) -> Self {
        assert!(radius >= T::ZERO);
        Self { center, radius }
    }
}

impl<T> Sdf<T> for Sphere<T>
where
    T: PrimitiveFloat,
{
    fn eval(&self, p: Vec3<T>) -> T {
        self.center.distance(p) - self.radius
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

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    static UNIT_SPHERE: LazyLock<Sphere<f32>> = LazyLock::new(|| Sphere::new(Vec3::ZERO, 1.0_f32));

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
        let sphere2 = Sphere::new(center2, 1.0);
        let sphere3 = Sphere::new(center3, 1.0);

        let combined = sphere.union(sphere2).union(sphere3);
        assert!(f32::abs(combined.eval(Vec3::ZERO) + 1.0) <= f32::EPSILON);
        assert!(f32::abs(combined.eval(center2) + 1.0) <= f32::EPSILON);
        assert!(f32::abs(combined.eval(center3) + 1.0) <= f32::EPSILON);
    }

    #[test]
    fn intersection_sdf() {
        let sphere = UNIT_SPHERE.clone();
        let sphere2 = Sphere::new(Vec3::X, 1.0);

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
        let sphere2 = Sphere::new(Vec3::X, 1.0);

        let subtracted = sphere.subtract(sphere2);
        assert!(f32::abs(subtracted.eval(Vec3::ZERO)) <= f32::EPSILON);
        assert!(f32::abs(subtracted.eval(-Vec3::X)) <= f32::EPSILON);
    }
}
