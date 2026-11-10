use crate::{interval::Interval, ray::Ray, vec3::Point3};

#[derive(Default, Clone, Copy)]
pub struct AABB {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl AABB {
    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        AABB { x, y, z }
    }

    pub fn from_point3s(a: Point3, b: Point3) -> Self {
        AABB {
            x: Interval::new(f64::min(a.x(), b.x()), f64::max(a.x(), b.x())),
            y: Interval::new(f64::min(a.y(), b.y()), f64::max(a.y(), b.y())),
            z: Interval::new(f64::min(a.z(), b.z()), f64::max(a.z(), b.z())),
        }
    }

    pub fn combine(box0: &AABB, box1: &AABB) -> Self {
        let x = Interval::combine(&box0.x, &box1.x);
        let y = Interval::combine(&box0.y, &box1.y);
        let z = Interval::combine(&box0.z, &box1.z);

        AABB { x, y, z }
    }

    pub fn axis_interval(&self, n: u32) -> Interval {
        match n {
            1 => self.y,
            2 => self.z,
            _ => self.x
        }
    }

    pub fn hit(&self, ray: &Ray, ray_t: &Interval) -> bool {
        let mut min_t = ray_t.min;
        let mut max_t = ray_t.max;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray.direction().get(axis as usize);

            let t0 = (ax.min - ray.origin().get(axis as usize)) * adinv;
            let t1 = (ax.max - ray.origin().get(axis as usize)) * adinv;

            if t0 < t1 {
                if t0 > min_t { min_t = t0 }
                if t1 < max_t { max_t = t1 }
            } else {
                if t1 > min_t { min_t = t1 }
                if t0 < max_t { max_t = t0 }
            }

            if max_t < min_t {
                return false
            }
        }
        true
    }
}
