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
mod checkered_texture;
mod rtw_image;
mod image_texture;
mod vector_math;

use vector_math::{random_f32, random_f32_within, random_vec3, random_vec3_within};
use checkered_texture::CheckerTexture;
use bvh::BVHNode;
use hittable::Hittable;
use image_texture::ImageTexture;
use std::{f32, fmt::Result};
use colour::Colour;
use dielectric::Dielectric;
use lambertian::Lambertian;
use material::Material;
use metal::Metal;
use std::sync::Arc;
use hittable_list::HittableList;
use sphere::Sphere;
use camera::Camera;

// no need to write your own Vector3
use nalgebra::{Point3, Vector3};


// SCENES 

fn bouncing_spheres() -> Result {
        //World
        let mut world = HittableList::new();
    
        //let ground_material = Box::new(Lambertian::new_from(Colour::new_from(0.5, 0.5, 0.5)));
        let checker = Box::new(CheckerTexture::new_from_colours(0.32, Colour::new_from(0.2, 0.3, 0.1), Colour::new_from(0.9, 0.9, 0.9)));
        let ground_material = Box::new(Lambertian::new_from_tex(checker));
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
    
        let _ = cam.render(&sync_world);
    
        Ok(())
}

fn checkered_spheres() -> Result {

    //World
    let mut world = HittableList::new();

    let checker = Box::new(CheckerTexture::new_from_colours(0.32, Colour::new_from(0.2, 0.3, 0.1), Colour::new_from(0.9, 0.9, 0.9)));
    let ground_material = Box::new(Lambertian::new_from_tex(checker));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-10.0,0.0), 10.0, ground_material.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.0,10.0,0.0), 10.0, ground_material.clone())));

    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    

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
    
        cam.defocus_angle = 0.0;
    
        let _ = cam.render(&sync_world);
    
        Ok(())
}

fn earth() -> Result {

    let mut world = HittableList::new();

    let earth_texture = Box::new(ImageTexture::new_from_filename("earthmap.jpg"));
    println!("Loaded image: {}x{}", &earth_texture.image.width(), &earth_texture.image.height());

    let earth_surface = Box::new(Lambertian::new_from_tex(earth_texture));
    world.add(Box::new(Sphere::new(Point3::new(0.0,0.0,0.0), 2.0, earth_surface.clone())));

    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    
    let mut cam = Camera::new();

    cam.aspect_ratio      = 16.0 / 9.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = 400.0;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 50; 

    cam.vfov     = 20;
    cam.lookfrom = Point3::new(0.0,0.0,12.0);
    cam.lookat   = Point3::new(0.0,0.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

fn quick_earth_test() -> Result {
    // 1) Build a single textured sphere at the origin
    let mut world = HittableList::new();
    let earth_tex = Box::new(ImageTexture::new_from_filename("earthmap.jpg"));
    let earth_mat = Box::new(Lambertian::new_from_tex(earth_tex));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_mat.clone(),
    )));
    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);

    // 2) Configure a super‑low‑res camera
    let mut cam = Camera::new();
    cam.aspect_ratio      = 16.0 / 9.0;
    cam.set_image_size(1200.0);//       = 800.0;    // << small!
    cam.samples_per_pixel = 1;        // << minimal AA
    cam.max_depth         = 2;        // << minimal bounces

    // Zoom in so you can actually see the sphere
    cam.vfov     = 20;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat   = Point3::new(0.0, 0.0, 0.0);
    cam.vup      = Vector3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    // 3) Render and inspect immediately
    let _ = cam.render(&sync_world);
    Ok(())
}


pub fn main() -> Result {

    match 3 {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => quick_earth_test(),
        _ => {todo!()}
    }
    
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