mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;
mod aabb;

use std::sync::Arc;

use camera::Camera;
use camera::CameraParams;
use colour::Colour;
use hittable_list::HittableList;
use material::Dielectric;
use material::Lambertian;
use material::Metal;
use rand::random;
use rand::random_range;
use sphere::Sphere;
use vec3::Point3;
use vec3::Vec3;

fn main() {
    // World

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        None,
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = random();
            let centre = Point3::new(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).length_squared() > 0.81 {
                let sphere: Sphere = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    let material = Arc::new(Lambertian::new(albedo));
                    let end_centre = Some(centre + Vec3::new(0.0, random_range(0.0..0.5), 0.0));
                    Sphere::new(centre, end_centre, 0.2, material)
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzz: f64 = random();
                    let material = Arc::new(Metal::new(albedo, fuzz));
                    Sphere::new(centre, None, 0.2, material)
                } else {
                    // glass
                    let material = Arc::new(Dielectric::new(1.5));
                    Sphere::new(centre, None, 0.2, material)
                };
                world.add(Box::new(sphere));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        None,
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        None,
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        None,
        1.0,
        material3,
    )));

    // Camera
    let cam_params = CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,
    };

    let camera = Camera::new(cam_params);

    camera.render(&world);
}
