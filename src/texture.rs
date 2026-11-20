use std::{array, fs::File, io::BufReader};

use jpeg_decoder::{Decoder, ImageInfo};
use rand::{random, random_range};
use rand::seq::SliceRandom;

use crate::vec3::{self, Vec3};
use crate::{colour::Colour, interval::Interval, vec3::Point3};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageTexture {
    pixels: Vec<u8>,
    width: u16,
    height: u16,
}

impl ImageTexture {
    const COLOUR_SCALE: f64 = 1.0 / 255.0;

    pub fn new(path: &str) -> Self {
        let file = File::open(path).expect(path);
        let mut decoder = Decoder::new(BufReader::new(file));
        let pixels = decoder.decode().expect("failed to decode image");
        let ImageInfo { width, height, ..} = decoder.info().unwrap();

        ImageTexture { pixels, width, height }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point3) -> Colour {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.height <= 0 {
            return Colour::new(0.0, 1.0, 1.0);
        }

        let u = Interval::new(0.0, 1.0).clamp(u as f64);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v as f64);

        let x = (u * self.width as f64) as u32;
        let y = (v * self.height as f64) as u32;
        let colour_pos: usize = ((y * self.width as u32 + x) * 3) as usize;

        let r = ImageTexture::COLOUR_SCALE * self.pixels[colour_pos] as f64;
        let g = ImageTexture::COLOUR_SCALE * self.pixels[colour_pos + 1] as f64;
        let b = ImageTexture::COLOUR_SCALE * self.pixels[colour_pos + 2] as f64;

        Colour::new(r, g, b)
    }
}

pub struct Perlin {
    rand_vec: [Vec3; Perlin::POINT_COUNT],
    perm_x: [usize; Self::POINT_COUNT],
    perm_y: [usize; Self::POINT_COUNT],
    perm_z: [usize; Self::POINT_COUNT],
    scale: f64,
}

impl Perlin {
    const POINT_COUNT: usize = 256;

    pub fn new(scale: f64) -> Self {
        let rand_vec: [Vec3; Self::POINT_COUNT] = array::from_fn(|_| vec3::unit_vector(Vec3::random_range(-1.0, 1.0)));
        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();
        
        Perlin { rand_vec, perm_x, perm_y, perm_z, scale }
    }

    fn generate_perm() -> [usize; Self::POINT_COUNT] { 
        let mut p: [usize; Self::POINT_COUNT] = array::from_fn(|i| i);
        let mut rng = rand::rng();
        p.shuffle(&mut rng);
        p
    }

    pub fn noise(&self, p: &Point3) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();

        let i = p.x().floor() as isize;
        let j = p.y().floor() as isize;
        let k = p.z().floor() as isize;
        let c: [[[Vec3; 3]; 3]; 3] = array::from_fn(|di| array::from_fn(|dj| array::from_fn(|dk|
            self.rand_vec[
                self.perm_x[((i + di as isize) & 255) as usize] ^
                self.perm_y[((j + dj as isize) & 255) as usize] ^
                self.perm_z[((k + dk as isize) & 255) as usize]
            ]
        )));

        self.trilinear_interp(c, u, v, w)
    }

    fn trilinear_interp(&self, c: [[[Vec3; 3]; 3]; 3], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i = i as f64;
                    let j = j as f64;
                    let k = k as f64;
                    let weight_v = Vec3::new(u - i, v - j, w - k);
                    accum += (i * uu + (1.0 - i) * (1.0 - uu))
                        * (j * vv + (1.0 - j) * (1.0 - vv))
                        * (k * ww + (1.0 - k) * (1.0 - ww))
                        * vec3::dot(&c[i as usize][j as usize][k as usize], &weight_v);
                }
            }
        }
        accum
    }

    pub fn turbulence(&self, p: &Point3, depth: u32) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

impl Texture for Perlin {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Colour {
        let p = self.scale * *p;
        Colour::pure_diagonal(0.5) * (1.0 + f64::sin(self.scale * p.z() + 10.0 * self.turbulence(&p, 7)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_image_texture() {
        let image_texture = ImageTexture::new("image-textures/sample-red-1x1.jpg");
        let expected = ImageTexture { 
            pixels: vec![254, 0, 0],
            width: 1,
            height: 1,
        };
        assert_eq!(image_texture, expected)
    }

    #[test]
    fn perlin() {
        let perlin = Perlin::new(4.0);

        assert_ne!(perlin.perm_x, perlin.perm_y)
    }
}
