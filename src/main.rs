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

use std::f64::consts::PI;
use std::rc::Rc;

use hittable_list::HittableList;
use vec3::Point3;
use sphere::Sphere;
use camera::Camera;
use material::Lambertian;
use material::Metal;
use colour::Colour;
use material::Dielectric;


fn main() {
    // World

    let mut world = HittableList::new();

    let r = f64::cos(PI / 4.0);

    let material_left = Rc::new(Lambertian::new(Colour::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Colour::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, material_left)));
    world.add(Box::new(Sphere::new(Point3::new( r, 0.0, -1.0), r, material_right)));
    // let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    // let material_centre = Rc::new(Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    // let material_left = Rc::new(Dielectric::new(1.5));
    // let material_bubble = Rc::new(Dielectric::new(1.0 / 1.5));
    // let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0));
    //
    // world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    // world.add(Box::new(Sphere::new(Point3::new( 0.0,  0.0, -1.2), 0.5, material_centre)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0,  0.0, -1.0), 0.5, material_left)));
    // world.add(Box::new(Sphere::new(Point3::new(-1.0,  0.0, -1.0), 0.4, material_bubble)));
    // world.add(Box::new(Sphere::new(Point3::new( 1.0,  0.0, -1.0), 0.5, material_right)));

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let vfov = 90.0;

    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov);

    camera.render(&world);
}
