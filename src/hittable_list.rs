use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let mut hit = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(hit_rec) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_rec.t;
                hit = Some(hit_rec);
            }
        }

        hit
    }
}
