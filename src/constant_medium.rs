use std::sync::Arc;

use rand::random;

use crate::{aabb::Aabb, colour::Colour, hittable::{HitRecord, Hittable}, interval::Interval, material::{Isotropic, Material}, ray::Ray, texture::Texture, vec3::Vec3};

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material>,
}

impl ConstantMedium {
    pub fn new(boundary: Box<dyn Hittable>, density: f64, tex: Box<dyn Texture>) -> Self {
        let neg_inv_density = -1. / density;
        let phase_function = Arc::new(Isotropic::new(tex));
        ConstantMedium { boundary, neg_inv_density, phase_function }
    }

    pub fn new_colour(boundary: Box<dyn Hittable>, density: f64, albedo: Colour) -> Self {
        let neg_inv_density = -1. / density;
        let phase_function = Arc::new(Isotropic::from_colour(albedo));
        ConstantMedium { boundary, neg_inv_density, phase_function }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if let Some(rec1) = self.boundary.hit(ray, Interval::UNIVERSE) {
            if let Some(rec2) = self.boundary.hit(ray, Interval::new(rec1.t + 0.0001, f64::INFINITY)) {
                let mut rec1 = rec1;
                let mut rec2 = rec2;
                
                if rec1.t < ray_t.min { rec1.t = ray_t.min }
                if rec2.t > ray_t.max { rec2.t = ray_t.max }

                if rec1.t >= rec2.t {
                    return None;
                }

                if rec1.t < 0. { rec1.t = 0. }

                let ray_length = ray.direction().length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random::<f64>().ln();

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = rec1.t + hit_distance / ray_length;
                Some(HitRecord { 
                    t, 
                    p: ray.at(t),
                    normal: Vec3::new(1., 0., 0.), // arbitrary
                    material: self.phase_function.clone(), 
                    u: 0., 
                    v: 0., 
                    front_face: true, // also arbitrary
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    fn bounding_box(&self) -> Aabb {
        self.boundary.bounding_box()
    }
}
