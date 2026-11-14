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

fn main() {
    scenes::moving_cover_bvh();
}
