use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

// type alias
pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour) {
    let r = pixel_colour.x();
    let g = pixel_colour.y();
    let b = pixel_colour.z();

    // Applying a linear to gamma transform for gamma 2
    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);


    let intensity = Interval::new(0.0, 0.999);
    let rbyte: i32 = (256.0 * intensity.clamp(r)) as i32;
    let gbyte: i32 = (256.0 * intensity.clamp(g)) as i32;
    let bbyte: i32 = (256.0 * intensity.clamp(b)) as i32;

    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).expect("writing colour");
}

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component < 0.0 {
        return 0.0;
    }

    linear_component.sqrt()
}
