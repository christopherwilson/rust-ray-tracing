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
}

impl Metal {
    pub fn new(albedo: Colour) -> Metal {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = vec3::reflect(r_in.direction(), rec.normal);
        Some(ScatterRecord { 
            attenuation: self.albedo, 
            scattered: Ray::new(rec.p, reflected) 
        })
    }
}
