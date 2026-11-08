mod common;
mod vec3;
mod colour;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod material;

use std::rc::Rc;

use hittable_list::HittableList;
use vec3::Point3;
use sphere::Sphere;
use camera::Camera;
use material::Lambertian;
use material::Metal;
use colour::Colour;


fn main() {
    // World

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_centre = Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    world.add(Box::new(Sphere::new(Point3::new( 0.0,  0.0, -1.2), 0.5, material_centre)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0,  0.0, -1.0), 0.5, material_left)));
    world.add(Box::new(Sphere::new(Point3::new( 1.0,  0.0, -1.0), 0.5, material_right)));

    let camera = Camera::default();

    camera.render(&world);
}
