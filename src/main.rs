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

use std::sync::Arc;

use hittable_list::HittableList;
use rand::random;
use vec3::Point3;
use sphere::Sphere;
use camera::Camera;
use material::Lambertian;
use material::Metal;
use colour::Colour;
use material::Dielectric;
use vec3::Vec3;
use camera::CameraParams;
use material::Material;


fn main() {
    // World

    let mut world = HittableList::new();

    let ground_material = Arc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat:f64 = random();
            let centre = Point3::new(
                a as f64 + 0.9 * random::<f64>(), 
                0.2, 
                b as f64 + 0.9 * random::<f64>()
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).length_squared() > 0.81 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Colour::random() * Colour::random();
                    Arc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Colour::random_range(0.5, 1.0);
                    let fuzz: f64 = random();
                    Arc::new(Metal::new(albedo, fuzz))
                } else {
                    // glass
                    Arc::new(Dielectric::new(1.5))
                };
                world.add(Box::new(Sphere::new(centre, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Arc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    // Camera
    let cam_params = CameraParams {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 500,
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
