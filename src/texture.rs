use crate::{colour::Colour, vec3::Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour;
}

pub struct SolidColour {
    albedo: Colour,
}

impl SolidColour {
    pub fn new(albedo: Colour) -> Self {
        SolidColour { albedo }
    }

    pub fn from_rgb(r: f64, g: f64, b: f64) -> Self {
        SolidColour { albedo: Colour::new(r, g, b) }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Colour {
        self.albedo
    }
}

pub struct CheckerTexture {
    inv_scale: f64,
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        CheckerTexture { inv_scale: 1.0 / scale, even, odd }
    }

    pub fn from_colours(scale: f64, even_colour: Colour, odd_colour: Colour) -> Self {
        CheckerTexture { 
            inv_scale: 1.0 / scale, 
            even: Box::new(SolidColour::new(even_colour)),
            odd: Box::new(SolidColour::new(odd_colour)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Colour {
        let x_int = (self.inv_scale * p.x()).floor() as i32;
        let y_int = (self.inv_scale * p.y()).floor() as i32;
        let z_int = (self.inv_scale * p.z()).floor() as i32;

        if (x_int + y_int + z_int) % 2 == 0 {
            self.even.value(u, v, p)
        } else {
            self.odd.value(u, v, p)
        }
    }
}
