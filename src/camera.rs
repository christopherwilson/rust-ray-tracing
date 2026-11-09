use crate::{colour::{self, Colour}, common::degrees_to_radians, hittable::Hittable, interval::Interval, material::ScatterRecord, ray::Ray, vec3::{self, Point3, Vec3}};

use std::{io, rc::Rc};

struct PixelDelta {
    u: Vec3,
    v: Vec3,
}

pub struct Camera {
    image_width: u32,
    image_height: u32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta: PixelDelta,
    samples_per_pixel: u32,
    max_depth: u32,
    defocus_disc_u: Vec3,
    defocus_disc_v: Vec3,
    defocus_angle: f64,
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

    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32, max_depth: u32, vfov: f64, lookfrom: Point3, lookat: Point3, vup: Vec3, defocus_angle: f64, focus_dist: f64) -> Camera {
        let image_height: u32 = (image_width as f64 / aspect_ratio) as u32;
        
        let center = lookfrom;

        // Viewport dimensions
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculating u, v, w, unit basiis vectors for the camera coordinate frame

        let w = vec3::unit_vector(lookfrom - lookat);
        let u = vec3::unit_vector(vec3::cross(&vup, &w));
        let v = vec3::cross(&w, &u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;

        let pixel_delta = PixelDelta {
            u: viewport_u / image_width as f64,
            v: viewport_v / image_height as f64,
        };

        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;

        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta.u + pixel_delta.v);

        // defocus disc basis vectors
        let defocus_radius = focus_dist * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disc_u = u * defocus_radius;
        let defocus_disc_v = v * defocus_radius;

        Camera { image_width, image_height, center, pixel00_loc, pixel_delta, samples_per_pixel, max_depth, defocus_disc_u, defocus_disc_v, defocus_angle }
    }

    fn ray_colour(ray: &Ray, depth: u32, world: &dyn Hittable) -> Colour {
        // If we've exceeded ray bounce limit, no more light is gathered
        if depth == 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if let Some(rec) = world.hit(ray, Interval::new(0.001, f64::INFINITY)) {
            if let Some(mat) = &rec.material {
                let mat = Rc::clone(mat);
                if let Some(ScatterRecord { attenuation, scattered }) = mat.scatter(ray, &rec) {
                    return attenuation * Camera::ray_colour(&scattered, depth - 1, world);
                }
            }
            return Colour::new(0.0, 0.0, 0.0);
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

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disc_sample()
        };
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(
            ray_origin,
            ray_direction
        )
    }

    pub fn defocus_disc_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disc();
        self.center + (p.x() * self.defocus_disc_u) + (p.y() * self.defocus_disc_v)
    }
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio = 1.0;
        let image_width = 100;
        let samples_per_pixel = 10;
        let max_depth = 10;

        let vfov = 90.0;
        let lookfrom = Point3::new(0.0, 0.0, 0.0);
        let lookat = Point3::new(0.0, 0.0, 0.0);
        let vup = Vec3::new(0.0, 0.0, 0.0);

        let defocus_angle = 0.0;
        let focus_dist = 10.0;

        Camera::new(aspect_ratio, image_width, samples_per_pixel, max_depth, vfov, lookfrom, lookat, vup, defocus_angle, focus_dist)
    }
}
