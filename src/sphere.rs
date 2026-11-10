use std::sync::Arc;

use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

pub struct Sphere {
    centre: Ray,
    radius: f64,
    material: Arc<dyn Material>
}

impl Sphere {
    pub fn new(centre: Point3, end_centre: Option<Point3>, radius: f64, material: Arc<dyn Material>) -> Sphere {
        let centre = if let Some(end_centre) = end_centre {
            Ray::new(centre, end_centre - centre, 0.0)
        } else {
            Ray::new(centre, Vec3::zero(), 0.0)
        };
        Sphere { centre, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let current_centre = self.centre.at(ray.time());
        let oc = current_centre - ray.origin();
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

        let mut rec = HitRecord {
            t: root,
            p: ray.at(root),
            material: self.material.clone(),
            normal: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (rec.p - current_centre) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        Some(rec)
    }
}
