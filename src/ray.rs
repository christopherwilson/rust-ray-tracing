use crate::vec3::{Point3, Vec3};

#[derive(Default, Debug, PartialEq)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
       Ray { orig: origin, dir: direction }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ray_origin() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(ray.origin(), Point3::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn ray_direction() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(ray.direction(), Vec3::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn ray_at_0() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(ray.at(0.0), Point3::new(0.0, 0.0, 0.0))
    }

    #[test]
    fn ray_at_1() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(ray.at(1.0), Point3::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn ray_at_5_5() {
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 1.0, 1.0));

        assert_eq!(ray.at(5.5), Point3::new(5.5, 5.5, 5.5))
    }

    #[test]
    fn ray_1_1_1_origin_neg_direction_at_1() {
        let ray = Ray::new(Point3::new(1.0, 1.0, 1.0), Vec3::new(-1.0, -1.0, -1.0));

        assert_eq!(ray.at(1.0), Point3::new(0.0, 0.0, 0.0))
    }
}
