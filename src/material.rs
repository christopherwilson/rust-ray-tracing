use crate::{colour::Colour, hittable::HitRecord, ray::Ray};

pub struct ScatterRecord {
    pub attenuation: Colour,
    pub scattered: Ray,
}

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
    ) -> Option<ScatterRecord>;
}
