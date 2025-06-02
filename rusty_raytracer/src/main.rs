mod scenes;
mod core;
mod geometry;
mod materials;
mod textures;
mod transforms;
mod util;
mod volumes;

use crate::scenes::{bouncing_spheres, checkered_spheres, cornell_box, cornell_smoke, earth, final_scene, perlin_spheres, quads, quick_earth_test, simple_light, test_inner_spheres_quick};

pub fn main() -> Result<(), ()> {

    match 11 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => quick_earth_test(),
        5 => perlin_spheres(),
        6 => quads(),
        7 => simple_light(),
        8 => cornell_box(),
        9 => cornell_smoke(),
        10 => final_scene(800.0, 10000, 40),
        11 => final_scene(400.0, 50, 4),
        12 => test_inner_spheres_quick(),
        _ => {todo!()}   
    }
    
}