use std::sync::Arc;

use crate::{aabb::Aabb, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray};

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BVHNode {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        let mut bbox = Aabb::default();

        for object in &objects {
            bbox = Aabb::combine(&bbox, &object.bounding_box());
        }

        let axis = bbox.longest_axis();

        let compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            let a_axis_interval = a.bounding_box().axis_interval(axis as u32);
            let b_axis_interval = b.bounding_box().axis_interval(axis as u32);
            a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
        };

        let num_objects = objects.len();

        let mut objects = objects;

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if num_objects == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if num_objects == 2 {
            left = objects[0].clone();
            right = objects[num_objects - 1].clone();
        } else {
            objects.sort_by(compare);

            let mid = num_objects / 2;
            left = Arc::new(BVHNode::new(objects[..mid].to_vec()));
            right = Arc::new(BVHNode::new(objects[mid..].to_vec()));
        }

        let bbox = Aabb::combine(&left.bounding_box(), &right.bounding_box());
        BVHNode { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<HitRecord> {
        if !self.bbox.hit(ray, &ray_t) {
            return None
        }

        if let Some(record) = self.left.hit(ray, ray_t) {
            // need to check if there is a closer hit on the right
            if let Some(record) = self.right.hit(ray, Interval::new(ray_t.min, record.t)) {
                return Some(record);
            }
            return Some(record);
        }

        if let Some(record) = self.right.hit(ray, ray_t) {
            return Some(record);
        }

        None
    } 

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
