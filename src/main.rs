mod common;
mod vec3;
mod colour;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;

use std::io;

use colour::Colour;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use crate::vec3::{Point3, Vec3};
use crate::ray::Ray;
use sphere::Sphere;
use interval::Interval;

fn ray_colour(ray: &Ray, world: &dyn Hittable) -> Colour {
    let mut rec = HitRecord::new();
    if world.hit(ray, Interval::new(0.0, f64::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Colour::new(1.0, 1.0, 1.0));
    }

    let unit_direction = vec3::unit_vector(ray.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    
    const IMAGE_WIDTH: u32 = 400;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    // Viewport

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (IMAGE_WIDTH as f64 / IMAGE_HEIGHT as f64);

    // World

    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera 

    let focal_length = 1.0;
    
    let camera_centre = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -VIEWPORT_HEIGHT, 0.0);

    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    let viewport_upper_left = camera_centre - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_centre = pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_centre - camera_centre;
            let r = Ray::new(camera_centre, ray_direction);

            let pixel_colour = ray_colour(&r, &world);
            colour::write_colour(&mut io::stdout(), pixel_colour);
        }
    }

    eprint!("\rDone.                 \n");
}
