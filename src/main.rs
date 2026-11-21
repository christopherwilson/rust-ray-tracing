mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod sphere;
mod vec3;
mod aabb;
mod bvh;
mod scenes;
mod texture;
mod quad;

fn main() {
    let scene = 6;

    match scene {
        1 => scenes::moving_cover_bvh(),
        2 => scenes::checkered_spheres(),
        3 => scenes::earth(),
        4 => scenes::perlin_spheres(),
        5 => scenes::quads(),
        6 => scenes::simple_light(),
        _ => ()
    }
    
}
