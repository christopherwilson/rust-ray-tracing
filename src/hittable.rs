use std::sync::Arc;

use crate::aabb::Aabb;
use crate::common::degrees_to_radians;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = vec3::dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord>;

    fn bounding_box(&self) -> Aabb;
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Translate { object, offset, bbox }
    }
}

impl Hittable for Translate {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let offset_ray = Ray::new(ray.origin() - self.offset, ray.direction(), ray.time());

        if let Some(hit_record) = self.object.hit(&offset_ray, ray_t) {
            let mut hit_record = hit_record;
            hit_record.p += self.offset;
            Some(hit_record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let newx =  cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min.set(c, min.get(c).min(tester.get(c)));
                        max.set(c, max.get(c).max(tester.get(c)));
                    }
                }
            }
        }

        let bbox = Aabb::from_point3s(min, max);

        RotateY { object, sin_theta, cos_theta, bbox }
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        // Transform the ray from worldd space to object space.

        let origin = Point3::new(
            (self.cos_theta * ray.origin().x()) - (self.sin_theta * ray.origin().z()), 
            ray.origin().y(), 
            (self.sin_theta * ray.origin().x()) + (self.cos_theta * ray.origin().z())
        );

        let direction = Point3::new(
            (self.cos_theta * ray.direction().x()) - (self.sin_theta * ray.direction().z()), 
            ray.direction().y(), 
            (self.sin_theta * ray.direction().x()) + (self.cos_theta * ray.direction().z())
        );

        let rotated_ray = Ray::new(origin, direction, ray.time());

        // Determine whether an intersection exists in object space (and if so, where).
        if let Some(record) = self.object.hit(&rotated_ray, ray_t) {
            // Transform the intersection from object space back to world space.

            let mut record = record;

            record.p = Point3::new(
                (self.cos_theta * record.p.x()) + (self.sin_theta * record.p.z()), 
                record.p.y(), 
                (-self.sin_theta * record.p.x()) + (self.cos_theta * record.p.z())
            );

            record.normal = Point3::new(
                (self.cos_theta * record.normal.x()) + (self.sin_theta * record.normal.z()), 
                record.normal.y(), 
                (-self.sin_theta * record.normal.x()) + (self.cos_theta * record.normal.z())
            );

            Some(record)
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
