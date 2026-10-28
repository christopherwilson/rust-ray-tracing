use std::io::Write;

use crate::vec3::Vec3;

// type alias
pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour) {
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();


    let rbyte: i32 = (255.999 * r) as i32;
    let gbyte: i32 = (255.999 * g) as i32;
    let bbyte: i32 = (255.999 * b) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("writing colour");
}
