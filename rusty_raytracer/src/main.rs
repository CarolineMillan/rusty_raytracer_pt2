mod colour;
mod ray;
mod hittable;
mod hittable_list;
mod sphere;
mod interval;
mod camera;
mod material;
mod lambertian;
mod metal;
mod dielectric;
mod aabb;
mod bvh;
mod texture;
mod solid_colour;

use bvh::BVHNode;
use hittable::Hittable;
use std::f32;
use colour::Colour;
use dielectric::Dielectric;
use lambertian::Lambertian;
use material::Material;
use metal::Metal;
use rand::{rng, Rng}; //random number generator
use std::sync::Arc;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;

// no need to write your own Vector3
use nalgebra::{Point3, Vector3};

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees*f32::consts::PI / 180.0
}

fn random_f32() -> f32 {
    let mut rng = rng();
    rng.random()
}

fn random_u32() -> u32 {
    let mut rng = rng();
    return rng.random::<u32>();
}

/*
fn random_u32_within(min: u32, max: u32) -> u32 {
    let mut rng = rng();
    let random_u32: u32 = rng.random::<u32>();
    return min + (max-min)*random_u32;
}
*/

fn random_u32_within(min: u32, max: u32) -> u32 {
    let mut rng = rng();
    let range = (max - min) as u64;
    let random_u32 = rng.random::<u32>() as u64;
    let scaled = (random_u32 * range) / (u32::MAX as u64 + 1);
    min + scaled as u32
}

fn random_f32_within(min: f32, max: f32) -> f32 {
    let mut rng = rng();
    let random_f32: f32 = rng.random();
    min + (max-min)*random_f32
}

fn random_vec3() -> Vector3<f32> {

    let x = random_f32();
    let y = random_f32();
    let z = random_f32();
    let random_vec = Vector3::new(x,y,z);
    random_vec
}

fn random_vec3_within(min: f32, max: f32) -> Vector3<f32> {
    // there's got to be a better way to do this with nalgebra
    let x = random_f32_within(min, max);
    let y = random_f32_within(min, max);
    let z = random_f32_within(min, max);

    let random_vec = Vector3::new(x,y,z);
    random_vec
}

fn random_unit_vector() -> Vector3<f32> {
    // not sure abt 1e-160 bit
    loop {
        let p = random_vec3_within(-1.0, 1.0);
        let lensq = p.norm_squared();
        if (1e-8 < lensq) && (lensq <= 1.0) {return p.normalize()}
    }
}
/*
fn random_on_hemisphere(normal: &Vector3<f32>) -> Vector3<f32> {
    let on_unit_sphere = random_unit_vector();
    if normal.dot(&on_unit_sphere) > 0.0 {return on_unit_sphere} else {return -on_unit_sphere}
}
*/
fn near_zero(vec: Vector3<f32>) -> bool {
    // is the vector nearzero in all directions?
    let s = 1e-8;
    (vec.x.abs() < s) && (vec.y.abs() < s) && (vec.z.abs() < s)
}

fn reflect(v: &Vector3<f32>, n: &Vector3<f32>) -> Vector3<f32> {
    v - 2.0*v.dot(n)*n
}

fn refract(uv: &Vector3<f32>, n: &Vector3<f32>, etai_over_etat: f32) -> Vector3<f32> {
    let cos_theta = f32::min(-uv.dot(n), 1.0);
    let r_out_perp = etai_over_etat * (uv + cos_theta*n);
    let r_out_parallel = -((1.0 - r_out_perp.norm_squared()).abs()).sqrt() * n;
    r_out_perp + r_out_parallel
}

fn random_in_unit_disk() -> Vector3<f32> {
    loop {
        let p = Vector3::new(random_f32_within(-1.0, 1.0), random_f32_within(-1.0, 1.0), 0.0);
        if p.norm_squared() < 1.0 {return p}
    }   
}

pub fn main() -> std::io::Result<()>{

    //World
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new_from(Colour::new_from(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material)));
    
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_f32();
            let center = Point3::new((a as f32)+0.9*random_f32(), 0.2, (b as f32)+0.9*random_f32());

            if (center - Point3::new(4.0, 0.2, 0.0)).len() as f32 > 0.9 {

                let sphere_material: Box<dyn Material>;

                if choose_mat < 0.8 {
                    //diffuse
                    let col_vec1 = random_vec3();
                    let col_vec2 = random_vec3();
                    let alb_col = Vector3::new(col_vec1[0]*col_vec2[0], col_vec1[1]*col_vec2[1], col_vec1[2]*col_vec2[2]);
                    let albedo = Colour::new_from(alb_col[0], alb_col[1], alb_col[2]);
                    sphere_material = Box::new(Lambertian::new_from(albedo));
                    let center2 = center + Vector3::new(0.0, random_f32(), 0.0);
                    world.add(Box::new(Sphere::new_moving(center, center2, 0.2, sphere_material)));
                }
                else if choose_mat < 0.95 {
                    //metal
                    let col_vec = random_vec3_within(0.5, 1.0);
                    let albedo = Colour::new_from(col_vec[0], col_vec[1], col_vec[2]);
                    let fuzz = random_f32_within(0.0, 0.5);
                    sphere_material = Box::new(Metal::new_from(albedo, fuzz));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
                else {
                    //glass
                    sphere_material = Box::new(Dielectric::new_from(1.5));
                    world.add(Box::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Box::new(Dielectric::new_from(1.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1)));

    let material2 = Box::new(Lambertian::new_from(Colour::new_from(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2)));

    let material3 = Box::new(Metal::new_from(Colour::new_from(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3)));

    println!("World has {} objects", &world.objects.len());
    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    //let sync_world = Arc::new(&world);

    println!("created sync world");

    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = 1200.0;
    cam.samples_per_pixel = 10;
    cam.max_depth         = 5;

    cam.vfov     = 20;
    cam.lookfrom = Point3::new(13.0,2.0,3.0);
    cam.lookat   = Point3::new(0.0,0.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist    = 10.0;

    println!("pre render");

    

    let _ = cam.render(&sync_world);

    println!("post render");

    Ok(())
}

/*

pub fn main() -> std::io::Result<()>{

    //World
    let mut world = HittableList::new();

    

    let material_ground = Box::new(Lambertian::new_from(Colour::new_from(0.8, 0.8, 0.0)));
    let material_center = Box::new(Lambertian::new_from(Colour::new_from(0.1, 0.2, 0.5)));
    let material_left = Box::new(Dielectric::new_from(1.50));
    let material_bubble = Box::new(Dielectric::new_from(1.0/1.50));
    let material_right = Box::new(Metal::new_from(Colour::new_from(0.8, 0.6, 0.2), 1.0));

    let sphere1 = Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground));
    let sphere2 = Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.2), 0.5, material_center));
    let sphere3 = Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left));
    let sphere4 = Box::new(Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.4, material_bubble));
    let sphere5 = Box::new(Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right));

    //tutorial uses "make_shared" here FIXME
    world.add(sphere1);
    world.add(sphere2);
    world.add(sphere3);
    world.add(sphere4);
    world.add(sphere5);

    
    /*
    let r = (f32::consts::PI/4.0).cos();

    let material_left = Box::new(Lambertian::new_from(Colour::new_from(0.0, 0.0, 1.0)));
    let material_right = Box::new(Lambertian::new_from(Colour::new_from(1.0, 0.0, 0.0)));

    let sphere1 = Box::new(Sphere::new(Point3::new(-r, 0.0, -1.0), r, material_left));
    let sphere2 = Box::new(Sphere::new(Point3::new(r, 0.0, -1.0), r, material_right));
  
    world.add(sphere1);
    world.add(sphere2);
    */


    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0/9.0;
    cam.image_width = 400.0;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vector3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    let _ = cam.render(&world);

    Ok(())
}

*/