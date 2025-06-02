use std::f32;

use nalgebra::Vector3;
use rand::{rng, Rng};


pub fn degrees_to_radians(degrees: f32) -> f32 {
    degrees*f32::consts::PI / 180.0
}

pub fn random_f32() -> f32 {
    let mut rng = rng();
    rng.random()
}
/*
pub fn random_u32() -> u32 {
    let mut rng = rng();
    return rng.random::<u32>();
}

pub fn random_u32_within(min: u32, max: u32) -> u32 {
    let mut rng = rng();
    let random_u32: u32 = rng.random::<u32>();
    return min + (max-min)*random_u32;
}

pub fn random_u32_within(min: u32, max: u32) -> u32 {
    let mut rng = rng();
    let range = (max - min) as u64;
    let random_u32 = rng.random::<u32>() as u64;
    let scaled = (random_u32 * range) / (u32::MAX as u64 + 1);
    min + scaled as u32
}
*/
pub fn random_f32_within(min: f32, max: f32) -> f32 {
    let mut rng = rng();
    let random_f32: f32 = rng.random();
    min + (max-min)*random_f32
}

pub fn random_vec3() -> Vector3<f32> {

    let x = random_f32();
    let y = random_f32();
    let z = random_f32();
    let random_vec = Vector3::new(x,y,z);
    random_vec
}

pub fn random_vec3_within(min: f32, max: f32) -> Vector3<f32> {
    // there's got to be a better way to do this with nalgebra
    let x = random_f32_within(min, max);
    let y = random_f32_within(min, max);
    let z = random_f32_within(min, max);

    let random_vec = Vector3::new(x,y,z);
    random_vec
}

pub fn random_unit_vector() -> Vector3<f32> {
    // not sure abt 1e-160 bit
    loop {
        let p = random_vec3_within(-1.0, 1.0);
        let lensq = p.norm_squared();
        if (1e-8 < lensq) && (lensq <= 1.0) {return p.normalize()}
    }
}
/*
pub fn random_on_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere = random_unit_vector();
    if normal.dot(&on_unit_sphere) > 0.0 {return on_unit_sphere} else {return -on_unit_sphere}
}
*/
pub fn near_zero(vec: Vector3<f32>) -> bool {
    // is the vector nearzero in all directions?
    let s = 1e-8;
    (vec.x.abs() < s) && (vec.y.abs() < s) && (vec.z.abs() < s)
}

pub fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0*v.dot(n)*n
}

pub fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = f32::min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

pub fn random_in_unit_disk() -> Vector3<f32> {
    loop {
        let p = Vector3::new(random_f32_within(-1.0, 1.0), random_f32_within(-1.0, 1.0), 0.0);
        if p.norm_squared() < 1.0 {return p}
    }   
}