use std::sync::Arc;

use crate::{aabb::Aabb, hittable::{HitRecord, Hittable}, interval::{self, Interval}, material::Material, vec3::{self, Point3, Vec3, cross, dot}};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    material: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, material: Arc<dyn Material>) -> Self {
        // bounding box
        let bbox_diag1 = Aabb::from_point3s(q, q + u + v);
        let bbox_diag2 = Aabb::from_point3s(q + u, q + v);
        let bbox = Aabb::combine(&bbox_diag1, &bbox_diag2);

        // normal and D
        let n = vec3::cross(&u, &v);
        let normal = vec3::unit_vector(n);
        let d = vec3::dot(&normal, &q);
        let w = n / dot(&n, &n);

        Quad { q, u, v, material, bbox, normal, d, w }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &crate::ray::Ray, ray_t: crate::interval::Interval) -> Option<HitRecord> {
        let denom = vec3::dot(&self.normal, &ray.direction());

        // No hit if the ray is aprallel to the plane
        if denom.abs() < 1e-8 { return None };

        let t = (self.d - dot(&self.normal, &ray.origin())) / denom;

        // If intersection occurs outside ray's lifetime no hit occurs
        if !ray_t.contains(t) { return None };

        // Determine if the hit point lies within the planar shape using its plane coorddinates.
        let intersection = ray.at(t);
        let planar_hitpt_vec = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitpt_vec, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vec));

        // The intersection point = alpha * u + beta * b, therefore if alpha or beta are not in
        // the interval [0..1] the intersection is not inside the quad so no hit.
        if !Interval::UNIT_INTERVAL.contains(alpha) || !Interval::UNIT_INTERVAL.contains(beta) { 
            return None 
        }

        let mut rec: HitRecord = HitRecord { 
            p: intersection, 
            normal: Default::default(), 
            material: self.material.clone(), 
            t, 
            u: alpha,
            v: beta,
            front_face: Default::default(), 
        };
        rec.set_face_normal(ray, self.normal);

        Some(rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
