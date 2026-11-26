mod aabb;
mod bvh;
mod camera;
mod colour;
mod common;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod quad;
mod ray;
mod scenes;
mod sphere;
mod texture;
mod vec3;
mod constant_medium;

fn main() {
    let scene = 0;

    match scene {
        1 => scenes::moving_cover_bvh(),
        2 => scenes::checkered_spheres(),
        3 => scenes::earth(),
        4 => scenes::perlin_spheres(),
        5 => scenes::quads(),
        6 => scenes::simple_light(),
        7 => scenes::cornell_box(),
        8 => scenes::cornell_smoke(),
        9 => scenes::final_scene(800, 10000, 40),
        _ => scenes::final_scene(400, 250, 4),

    }
}
