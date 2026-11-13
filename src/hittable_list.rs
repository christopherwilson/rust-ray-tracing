use crate::aabb::AABB;
use crate::hittable::{Hit, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    bbox: AABB,
}

impl HittableList {
    pub fn new() -> HittableList {
        Default::default()
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = AABB::combine(&self.bbox, &object.bounding_box());
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        let mut hit = None;
        let mut closest_so_far = ray_t.max;

        for object in &self.objects {
            if let Some(Hit::Record(hit_rec)) = object.hit(ray, Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_rec.t;
                hit = Some(Hit::Record(hit_rec));
            }
        }

        hit
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
