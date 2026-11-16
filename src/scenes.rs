use std::sync::Arc;

use crate::camera::Camera;
use crate::camera::CameraParams;
use crate::colour::Colour;
use crate::hittable_list::HittableList;
use crate::material::Dielectric;
use crate::material::Lambertian;
use crate::material::Metal;
use crate::texture::CheckerTexture;
use crate::texture::ImageTexture;
use crate::texture::SolidColour;
use rand::random;
use rand::random_range;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use crate::vec3::Vec3;
use crate::hittable::Hittable;
use crate::bvh::BVHNode;

pub fn moving_cover_bvh() {
    // World

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Box::new(SolidColour::new(Colour::new(0.5, 0.5, 0.5)))));
    world.push(Arc::new(Sphere::new(
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
                    let albedo = Box::new(SolidColour::new(Colour::random() * Colour::random()));
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
                world.push(Arc::new(sphere));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        None,
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Box::new(SolidColour::new(Colour::new(0.4, 0.2, 0.1)))));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        None,
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        None,
        1.0,
        material3,
    )));

    let world = BVHNode::new(world);

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

pub fn checker_sphere_floor() {
    // World

    let mut world: Vec<Arc<dyn Hittable>> = Vec::new();

    let ground_material = Arc::new(Lambertian::new(Box::new(CheckerTexture::from_colours(0.32, Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)))));
    world.push(Arc::new(Sphere::new(
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
                    let albedo = Box::new(SolidColour::new(Colour::random() * Colour::random()));
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
                world.push(Arc::new(sphere));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.push(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        None,
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Box::new(SolidColour::new(Colour::new(0.4, 0.2, 0.1)))));
    world.push(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        None,
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.push(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        None,
        1.0,
        material3,
    )));

    let world = BVHNode::new(world);

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

pub fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(Lambertian::new(Box::new(CheckerTexture::from_colours(0.32, Colour::new(0.2, 0.3, 0.1), Colour::new(0.9, 0.9, 0.9)))));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0), 
        None, 
        10.0, 
        checker.clone()
    )));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0), 
        None, 
        10.0, 
        checker.clone()
    )));

    let camera_params = CameraParams { 
        aspect_ratio: 16.0 / 9.0, 
        image_width: 400, 
        samples_per_pixel: 100, 
        max_depth: 50, 

        vfov: 20.0, 
        lookfrom: Point3::new(13.0, 2.0, 3.0), 
        lookat: Point3::zero(), 
        vup: Point3::new(0.0, 1.0, 0.0), 

        defocus_angle: 0.0, 

        focus_dist: 10.0 
    };

    let cam = Camera::new(camera_params);
    cam.render(&world);
}

pub fn earth() {
    let earth_texture = ImageTexture::new("image-textures/earthmap.jpg");
    let earth_material = Lambertian::new(Box::new(earth_texture));
    let globe = Sphere::new(Point3::zero(), None, 2.0, Arc::new(earth_material));

    let mut world = HittableList::new();
    world.add(Box::new(globe));

    let camera_params: CameraParams = CameraParams { 
        aspect_ratio: 16.0 / 9.0, 
        image_width: 400, 
        samples_per_pixel: 100, 
        max_depth: 50, 

        vfov: 20.0,
        lookfrom: Point3::new(0.0, 0.0, 12.0), 
        lookat: Point3::zero(), 
        vup: Point3::new(0.0, 1.0, 0.0), 
        defocus_angle: 0.0, 
        focus_dist: 10.0 
    };

    let camera = Camera::new(camera_params);

    camera.render(&world);
}
