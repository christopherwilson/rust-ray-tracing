use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3};

pub struct Sphere {
    centre: Point3,
    radius: f64,
    // material: Rc<dyn Material>
}

impl Sphere {
    // pub fn new(centre: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
    //     Sphere { centre, radius, material }
    // }
    pub fn new(centre: Point3, radius: f64) -> Sphere {
        Sphere { centre, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.centre - ray.origin();
        let a = ray.direction().length_squared();
        let h = vec3::dot(&ray.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::new();
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        // rec.material = Some(self.material.clone());
        Some(rec)
    }
}
