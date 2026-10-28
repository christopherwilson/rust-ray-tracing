mod vec3;
mod colour;
mod ray;

use std::io;

use colour::Colour;

fn main() {
    // Image
    
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in 0..IMAGE_HEIGHT {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b: f64 = 0.0;

            let pixel_colour = Colour::new(r, g, b);
            colour::write_colour(&mut io::stdout(), pixel_colour);
        }
    }

    eprint!("\rDone.                 \n");
}
