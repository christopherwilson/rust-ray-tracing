use std::{cmp::Ordering, sync::Arc};

use rand::{random, random_range};

use crate::{aabb::AABB, hittable::{Hit, Hittable}, interval::Interval, ray::Ray};

struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: Vec<Arc<dyn Hittable>>) -> Self {
        let axis: usize = random_range(0..=2);

        let mut objects = objects;

        let compare = |a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>| {
            let a_axis_interval = a.bounding_box().axis_interval(axis as u32);
            let b_axis_interval = b.bounding_box().axis_interval(axis as u32);
            a_axis_interval.min.partial_cmp(&b_axis_interval.min).unwrap()
        };

        let object_span = objects.len();

        let left: Arc<dyn Hittable>;
        let right: Arc<dyn Hittable>;

        if object_span == 1 {
            left = objects[0].clone();
            right = objects[0].clone();
        } else if object_span == 2 {
            left = objects[0].clone();
            right = objects[object_span].clone();
        } else {
            objects.sort_by(compare);

            let mid = 0 + object_span / 2;
            left = Arc::new(BVHNode::new(objects[0..mid].to_vec()));
            right = Arc::new(BVHNode::new(objects[mid..object_span].to_vec()));
        }

        let bbox = AABB::combine(&left.bounding_box(), &right.bounding_box());
        BVHNode { left, right, bbox }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: Interval) -> Option<Hit> {
        if !self.bbox.hit(ray, &ray_t) {
            return None
        }

        if let Some(Hit::Hit) = self.left.hit(ray, ray_t) {
            return Some(Hit::Hit);
        }

        if let Some(Hit::Hit) = self.right.hit(ray, ray_t) {
            return Some(Hit::Hit);
        }

        None
    } 

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}
