mod common;
mod vec3;
mod colour;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;

use hittable_list::HittableList;
use crate::vec3::Point3;
use sphere::Sphere;
use camera::Camera;


fn main() {
    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::default();

    camera.render(&world);
}
