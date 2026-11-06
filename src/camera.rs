use crate::{colour::{self, Colour}, hittable::{HitRecord, Hittable}, interval::Interval, ray::Ray, vec3::{self, Point3, Vec3}};

use std::io;

struct PixelDelta {
    u: Vec3,
    v: Vec3,
}

pub struct Camera {
    aspect_ratio: f64,
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta: PixelDelta,
    samples_per_pixel: u32,
    max_depth: u32,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 400;
        let samples_per_pixel = 100;
        let max_depth = 50;

        Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth)
    }
}

impl Camera {
    pub fn render(&self, world: &dyn Hittable) {
        // Render

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", j);
            for i in 0..self.image_width {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i as f64, j as f64);
                    pixel_colour += Camera::ray_colour(&r, self.max_depth, world)
                }
                colour::write_colour(&mut io::stdout(), pixel_colour / self.samples_per_pixel as f64);
            }
        }

        eprint!("\rDone.                 \n");
    }

    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32) -> Camera {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        
        let center = Point3::new(0.0, 0.0, 0.0);

        // Viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        let pixel_delta = PixelDelta {
            u: viewport_u / image_width as f64,
            v: viewport_v / image_height as f64,
        };

        let viewport_upper_left = center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta.u + pixel_delta.v);

        Camera { aspect_ratio, image_width, image_height, center, pixel00_loc, pixel_delta, samples_per_pixel, max_depth }
    }

    fn ray_colour(ray: &Ray, depth: u32, world: &dyn Hittable) -> Colour {
        // If we've exceeded ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::new();
        if world.hit(ray, Interval::new(0.001, f64::INFINITY), &mut rec) {
            let direction = rec.normal + Vec3::random_unit_vector();
            return 0.5 * Camera::ray_colour(&Ray::new(rec.p, direction), depth - 1, world);
        }

        let unit_direction = vec3::unit_vector(ray.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - a) * Colour::new(1.0, 1.0, 1.0) + a * Colour::new(0.5, 0.7, 1.0)
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let offset = Vec3::sample_square();
        let pixel_sample = self.pixel00_loc 
                           + ((u + offset.x()) * self.pixel_delta.u)
                           + ((v + offset.y()) * self.pixel_delta.v);
        Ray::new(
            self.center, 
            pixel_sample - self.center
        )
    }
}
