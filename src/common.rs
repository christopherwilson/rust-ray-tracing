// Constants

pub use std::f64::consts::PI;

// Utility functions

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn fmin(a: f64, b:f64) -> f64 {
    if a <= b {
        a
    } else {
        b
    }
}

pub fn fmax(a: f64, b:f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}
