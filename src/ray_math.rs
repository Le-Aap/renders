use crate::vec_math::Vec3;

#[derive(Debug, PartialEq)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn at(&self, t:f64) -> Vec3 {
        *self.origin() + *self.direction() * t
    }

    pub fn origin(&self) -> &Vec3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
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
        let point = ray.at(2.0);
        assert_eq!(point, Vec3::new(2.0, 0.0, 0.0));
        assert_eq!(origin, *ray.origin());
        assert_eq!(direction.normalized(), *ray.direction());
    }
}