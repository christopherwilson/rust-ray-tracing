use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

// type alias
pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour) {
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();


    let intensity = Interval::new(0.0, 0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("writing colour");
}
