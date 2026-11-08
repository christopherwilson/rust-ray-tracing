use rand::random;

use crate::{colour::Colour, hittable::HitRecord, ray::Ray, vec3::{self, Vec3}};

pub struct ScatterRecord {
    pub attenuation: Colour,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let scatter_direction = rec.normal + Vec3::random_unit_vector();
        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };
            
        Some(ScatterRecord { 
            attenuation: self.albedo, 
            scattered: Ray::new(rec.p, scatter_direction),
        })
    }
}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Colour, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(r_in.direction(), rec.normal);
        let fuzzy_reflected = vec3::unit_vector(reflected) + (self.fuzz * Vec3::random_unit_vector());
        Some(ScatterRecord { 
            attenuation: self.albedo, 
            scattered: Ray::new(rec.p, fuzzy_reflected) 
        })
    }
}

pub struct Dielectric {
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(&self, cosine: f64) -> f64 {
        let r0 = (1.0 - self.refraction_index) / (1.0 + self.refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Colour::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = vec3::unit_vector(r_in.direction());
        let cos_theta = f64::min(vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        
        let cant_refract = ri * sin_theta > 1.0;

        let direction = if cant_refract || self.reflectance(cos_theta) > random() {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, ri)
        };

        let scattered = Ray::new(rec.p, direction);

        Some(ScatterRecord { attenuation, scattered })
    }
}
