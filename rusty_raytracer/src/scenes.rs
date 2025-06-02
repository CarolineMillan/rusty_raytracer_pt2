// SCENES 

use std::sync::Arc;

use nalgebra::{Point3, Vector3};

use crate::core::camera::Camera;
use crate::materials::dielectric::Dielectric;
use crate::materials::diffuse_light::DiffuseLight;
use crate::materials::material::Material;
use crate::materials::metal::Metal;
use crate::textures::image_texture::ImageTexture;
use crate::textures::noise_texture::NoiseTexture;
use crate::transforms::rotate_y::RotateY;
use crate::transforms::translate::Translate;
use crate::util::vector_math::{random_f32, random_f32_within, random_vec3, random_vec3_within};
use crate::volumes::constant_medium::ConstantMedium;
use crate::{core::colour::Colour, geometry::hittable_list::HittableList, materials::lambertian::Lambertian, textures::checkered_texture::CheckerTexture};
use crate::geometry::sphere::Sphere;
use crate::geometry::bvh::BVHNode;
use crate::geometry::hittable::Hittable;
use crate::geometry::quad::Quad;
use crate::geometry::quad::make_box;

pub fn bouncing_spheres() -> Result<(), ()> {
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
        cam.background = Colour::new_from(0.7, 0.8, 1.0);
    
        cam.vfov     = 20;
        cam.lookfrom = Point3::new(13.0,2.0,3.0);
        cam.lookat   = Point3::new(0.0,0.0,0.0);
        cam.vup      = Vector3::new(0.0,1.0,0.0);
    
        cam.defocus_angle = 0.6;
        cam.focus_dist    = 10.0;
    
        let _ = cam.render(&sync_world);
    
        Ok(())
}

pub fn checkered_spheres() -> Result<(), ()> {

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
        cam.background = Colour::new_from(0.7, 0.8, 1.0);
    
        cam.vfov     = 20;
        cam.lookfrom = Point3::new(13.0,2.0,3.0);
        cam.lookat   = Point3::new(0.0,0.0,0.0);
        cam.vup      = Vector3::new(0.0,1.0,0.0);
    
        cam.defocus_angle = 0.0;
    
        let _ = cam.render(&sync_world);
    
        Ok(())
}

pub fn earth() -> Result<(), ()> {

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
    cam.background = Colour::new_from(0.7, 0.8, 1.0);

    cam.vfov     = 20;
    cam.lookfrom = Point3::new(0.0,0.0,12.0);
    cam.lookat   = Point3::new(0.0,0.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

pub fn quick_earth_test() -> Result<(), ()> {
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
    cam.background = Colour::new_from(0.7, 0.8, 1.0);

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

pub fn perlin_spheres() -> Result<(), ()> {

    //World
    let mut world = HittableList::new();

    let pertext = Box::new(NoiseTexture::new(4.0));
    let ground_material = Box::new(Lambertian::new_from_tex(pertext));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.0,2.0,0.0), 2.0, ground_material.clone())));

    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    

    let mut cam = Camera::new();
    
    cam.aspect_ratio      = 16.0 / 9.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = 1200.0;
    cam.samples_per_pixel = 10;
    cam.max_depth         = 5;
    cam.background = Colour::new_from(0.7, 0.8, 1.0);

    cam.vfov     = 20;
    cam.lookfrom = Point3::new(13.0,2.0,3.0);
    cam.lookat   = Point3::new(0.0,0.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

pub fn quads() -> Result<(), ()> {

    // Create World
    let mut world = HittableList::new();

    //let pertext = Box::new(NoiseTexture::new(4.0));

    // Materials
    let left_red = Box::new(Lambertian::new_from(Colour::new_from(1.0, 0.2, 0.2)));
    let back_green = Box::new(Lambertian::new_from(Colour::new_from(0.2, 1.0, 0.2)));
    let right_blue = Box::new(Lambertian::new_from(Colour::new_from(0.2, 0.2, 1.0)));
    let upper_orange = Box::new(Lambertian::new_from(Colour::new_from(1.0, 0.5, 0.0)));
    let lower_teal = Box::new(Lambertian::new_from(Colour::new_from(0.2, 0.8, 0.8)));
    
    //Quads
    world.add(Box::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vector3::new(0.0, 0.0, -4.0), Vector3::new(0.0, 4.0, 0.0), left_red)));
    world.add(Box::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 4.0, 0.0), back_green)));
    world.add(Box::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Vector3::new(0.0, 0.0, 4.0), Vector3::new(0.0, 4.0, 0.0), right_blue)));
    world.add(Box::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 4.0), upper_orange)));
    world.add(Box::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vector3::new(4.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -4.0), lower_teal)));

    // bounding boxes
    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    

    //Camera
    let mut cam = Camera::new();
    
    cam.aspect_ratio      = 1.0; //16.0 / 9.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = 1200.0;
    cam.samples_per_pixel = 10;
    cam.max_depth         = 5;
    cam.background = Colour::new_from(0.7, 0.8, 1.0);

    cam.vfov     = 80;
    cam.lookfrom = Point3::new(0.0,0.0,9.0);
    cam.lookat   = Point3::new(0.0,0.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

pub fn simple_light() -> Result<(), ()> {

    //World
    let mut world = HittableList::new();

    let pertext = Box::new(NoiseTexture::new(4.0));
    let ground_material = Box::new(Lambertian::new_from_tex(pertext));
    world.add(Box::new(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.0,2.0,0.0), 2.0, ground_material.clone())));

    let difflight = Box::new(DiffuseLight::new_from(Colour::new_from(4.0, 4.0, 4.0)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight.clone())));
    world.add(Box::new(Quad::new(Point3::new(3.0, 1.0,  -2.0), Vector3::new(2.0, 0.0, 0.0), Vector3::new(0.0, 2.0, 0.0), difflight.clone())));

    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    

    let mut cam = Camera::new();
    
    cam.aspect_ratio      = 16.0 / 9.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = 1200.0;
    cam.samples_per_pixel = 10;
    cam.max_depth         = 5;
    cam.background = Colour::new_from(0.0, 0.0, 0.0);

    cam.vfov     = 20;
    cam.lookfrom = Point3::new(26.0,3.0,6.0);
    cam.lookat   = Point3::new(0.0,2.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

pub fn cornell_box() -> Result<(), ()> {

    //World
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::new_from(Colour::new_from(0.65, 0.05, 0.05)));
    let white = Box::new(Lambertian::new_from(Colour::new_from(0.73, 0.73, 0.73)));
    let green = Box::new(Lambertian::new_from(Colour::new_from(0.12, 0.45, 0.15)));
    let light = Box::new(DiffuseLight::new_from(Colour::new_from(15.0, 15.0, 15.0)));

    world.add(Box::new(Quad::new(Point3::new(555.0, 0.0,  0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), green.clone())));
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0,  0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), red.clone())));
    world.add(Box::new(Quad::new(Point3::new(343.0, 554.0,  332.0), Vector3::new(-130.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -105.0), light.clone())));
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0,  0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Box::new(Quad::new(Point3::new(555.0, 555.0,  555.0), Vector3::new(-555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, -555.0), white.clone())));
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0,  555.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = Box::new(make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1_rotated = Box::new(RotateY::new(box1.clone(), 15.0));
    let box1_translated = Box::new(Translate::new(box1_rotated.clone(), Vector3::new(265.0,  0.0, 295.0)));
    world.add(box1_translated);

    let box2 = Box::new(make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), white.clone()));
    let box2_rotated = Box::new(RotateY::new(box2.clone(), -18.0));
    let box2_translated = Box::new(Translate::new(box2_rotated.clone(), Vector3::new(130.0, 0.0, 65.0)));
    world.add(box2_translated);

    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    

    let mut cam = Camera::new();
    
    cam.aspect_ratio      = 1.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = 600.0;
    cam.samples_per_pixel = 100;
    cam.max_depth         = 5;
    cam.background = Colour::new_from(0.0, 0.0, 0.0);

    cam.vfov     = 40;
    cam.lookfrom = Point3::new(278.0,278.0,-800.0);
    cam.lookat   = Point3::new(278.0,278.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

pub fn cornell_smoke() -> Result<(), ()> {

    //World
    let mut world = HittableList::new();

    let red = Box::new(Lambertian::new_from(Colour::new_from(0.65, 0.05, 0.05)));
    let white = Box::new(Lambertian::new_from(Colour::new_from(0.73, 0.73, 0.73)));
    let green = Box::new(Lambertian::new_from(Colour::new_from(0.12, 0.45, 0.15)));
    let light = Box::new(DiffuseLight::new_from(Colour::new_from(7.0, 7.0, 7.0)));

    world.add(Box::new(Quad::new(Point3::new(555.0, 0.0,  0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), green.clone())));
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0,  0.0), Vector3::new(0.0, 555.0, 0.0), Vector3::new(0.0, 0.0, 555.0), red.clone())));
    world.add(Box::new(Quad::new(Point3::new(113.0, 554.0,  127.0), Vector3::new(330.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 305.0), light.clone())));
    world.add(Box::new(Quad::new(Point3::new(0.0, 555.0,  0.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Box::new(Quad::new(Point3::new(000.0, 0.0,  000.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 555.0), white.clone())));
    world.add(Box::new(Quad::new(Point3::new(0.0, 0.0,  555.0), Vector3::new(555.0, 0.0, 0.0), Vector3::new(0.0, 555.0, 0.0), white.clone())));

    let box1 = Box::new(make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 330.0, 165.0), white.clone()));
    let box1_rotated = Box::new(RotateY::new(box1.clone(), 15.0));
    let box1_translated = Box::new(Translate::new(box1_rotated.clone(), Vector3::new(265.0,  0.0, 295.0)));
    world.add(Box::new(ConstantMedium::new_from_colour(box1_translated, 0.01, Colour::new_from(0.0, 0.0, 0.0))));

    let box2 = Box::new(make_box(&Point3::new(0.0, 0.0, 0.0), &Point3::new(165.0, 165.0, 165.0), white.clone()));
    let box2_rotated = Box::new(RotateY::new(box2.clone(), -18.0));
    let box2_translated = Box::new(Translate::new(box2_rotated.clone(), Vector3::new(130.0, 0.0, 65.0)));
    world.add(Box::new(ConstantMedium::new_from_colour(box2_translated, 0.01, Colour::new_from(1.0, 1.0, 1.0))));


    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    

    let mut cam = Camera::new();
    
    cam.aspect_ratio      = 1.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = 600.0;
    cam.samples_per_pixel = 200;
    cam.max_depth         = 50;
    cam.background = Colour::new_from(0.0, 0.0, 0.0);

    cam.vfov     = 40;
    cam.lookfrom = Point3::new(278.0,278.0,-800.0);
    cam.lookat   = Point3::new(278.0,278.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

pub fn final_scene(image_width: f32, samples_per_pixel: u32, max_depth: u32) -> Result<(), ()> {

    println!("In final_scene with image_width = {}, samples_per_pixel = {}, and max_depth = {}", image_width, samples_per_pixel, max_depth);

    //Create a world
    let mut boxes1 = HittableList::new();

    // ground material
    let ground = Box::new(Lambertian::new_from(Colour::new_from(0.48, 0.83, 0.53)));
    
    // make all the boxes for the ground
    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32*w;
            let y0 = 0.0;
            let z0 = -1000.0 + j as f32*w;           
            let x1 = x0 + w;
            let y1 = random_f32_within(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Box::new(make_box(&Point3::new(x0, y0, z0), &Point3::new(x1, y1, z1), ground.clone())));
        }
    }

    //Create a world
    let mut world = HittableList::new();
    world.add(Box::new(boxes1));
    
    // Light
    let light = Box::new(DiffuseLight::new_from(Colour::new_from(7.0, 7.0, 7.0)));
    world.add(Box::new(Quad::new(Point3::new(123.0, 554.0,  147.0), Vector3::new(300.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 265.0), light.clone())));

    //lambertian moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vector3::new(30.0, 0.0, 0.0);
    let sphere_material = Box::new(Lambertian::new_from(Colour::new_from(0.7, 0.3, 0.1)));
    world.add(Box::new(Sphere::new_moving(center1, center2, 50.0, sphere_material)));
    
    // glass sphere
    let glass = Box::new(Dielectric::new_from(1.5));
    world.add(Box::new(Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, glass.clone())));

    // metal sphere
    world.add(Box::new(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Box::new(Metal::new_from(Colour::new_from(0.8, 0.8, 0.9), 1.0)))));

    // volume stuff -- with a glass sphere around it? (volume inside a dielectric is what a subsurface material is, blue sphere in the image)
    let boundary = Box::new(Sphere::new(Point3::new(360.0, 150.0, 145.0), 75.0, glass.clone()));
    world.add(boundary.clone());

    world.add(Box::new(ConstantMedium::new_from_colour(boundary.clone(), 0.2, Colour::new_from(1.0, 1.0, 1.0))));

    // thin layer of fog over the scene
    let boundary2 = Box::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, glass.clone()));
    world.add(Box::new(ConstantMedium::new_from_colour(boundary2.clone(), 0.0001, Colour::new_from(1.0, 1.0, 1.0))));
    
    // planet earth
    let earth_tex = Box::new(ImageTexture::new_from_filename("earthmap.jpg"));
    let earth_mat = Box::new(Lambertian::new_from_tex(earth_tex));
    world.add(Box::new(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, earth_mat.clone())));

    // perlin textured sphere
    let pertext = Box::new(NoiseTexture::new(0.2));
    let pertext_material = Box::new(Lambertian::new_from_tex(pertext));
    world.add(Box::new(Sphere::new(Point3::new(220.0,280.0,300.0), 80.0, pertext_material.clone())));

    // make a box of spheres, translate and rotate them before adding to our main hittable list
    let mut boxes2 = HittableList::new();
    let white = Box::new(Lambertian::new_from(Colour::new_from(0.73, 0.73, 0.73)));
    let ns = 1000; // number of spheres
    // FIXME -- there's probaably a better way to loop here if you don't use j
    for _j in 0..ns {
        boxes2.add(Box::new(Sphere::new(Point3::new(random_f32_within(0.0, 165.0), random_f32_within(0.0, 165.0), random_f32_within(0.0, 165.0)), 10.0, white.clone())));
    }

    // THIS IS THE LINE I'M TAKING ABOUT, CHATGPT!
    world.add(Box::new(Translate::new(Box::new(RotateY::new(Box::new(BVHNode::from_hittable_list(boxes2)), 15.0)), Vector3::new(-100.0, 270.0, 395.0))));


    // make BVH from the world we've just created
    let world_bbox  = BVHNode::from_hittable_list(world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);
    

    // ********************* CAMERA ***********************
    let mut cam = Camera::new();
    
    cam.aspect_ratio      = 1.0;
    // keep width at 1200, it doesn't work at 400
    cam.image_width       = image_width;
    cam.samples_per_pixel = samples_per_pixel;
    cam.max_depth         = max_depth;
    cam.background = Colour::new_from(0.0, 0.0, 0.0);

    cam.vfov     = 40;
    cam.lookfrom = Point3::new(478.0,278.0,-600.0);
    cam.lookat   = Point3::new(278.0,278.0,0.0);
    cam.vup      = Vector3::new(0.0,1.0,0.0);

    cam.defocus_angle = 0.0;

    let _ = cam.render(&sync_world);

    Ok(())
}

pub fn test_inner_spheres_quick() -> Result<(), ()> {
    // 1. Recompute just the cluster BVH (boxes2) and its transforms:
    let mut boxes2 = HittableList::new();
    // Light
    let light = Box::new(DiffuseLight::new_from(Colour::new_from(7.0, 7.0, 7.0)));
    boxes2.add(Box::new(Quad::new(Point3::new(123.0, 554.0,  147.0), Vector3::new(300.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 265.0), light.clone())));

    let white = Box::new(Lambertian::new_from(Colour::new_from(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Box::new(Sphere::new(
            Point3::new(random_f32_within(0.0, 165.0),
                        random_f32_within(0.0, 165.0),
                        random_f32_within(0.0, 165.0)),
            10.0,
            white.clone(),
        )));
    }
    // … inside final_scene (or your test function), right after filling boxes2:
    let raw_boxes2_bbox = boxes2.bounding_box();
    println!(
        "\n[Debug] boxes2.bounding_box()  →  {:?}\n",
        raw_boxes2_bbox
    );

    let inner_bvh = BVHNode::from_hittable_list(boxes2);
    let moved_bvh = Translate::new(
        Box::new(RotateY::new(Box::new(inner_bvh), 15.0)),
        Vector3::new(-100.0, 270.0, 395.0),
    );

    let moved_bbox = moved_bvh.bounding_box();
    println!(
        "[Debug] moved_bvh (RotateY + Translate) → {:?}\n",
        moved_bbox
    );

    
    // 2. Wrap that single Hittable in a top-level HittableList so we can trace it:
    let mut test_world = HittableList::new();
    test_world.add(Box::new(moved_bvh));     // no other objects

    // 3. Build a camera that focuses on the bounding box you just printed:
    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width  = 200.0;                // very low resolution
    cam.samples_per_pixel = 10;              // super low SPP
    cam.max_depth = 2;                       // no deep recursion
    cam.background = Colour::new_from(0.0, 0.0, 0.0);

    cam.vfov   = 40;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(   1.0, 352.0, 454.0  );
    //cam.lookat   = Point3::new(278.0, 278.0,   0.0);
    cam.vup      = Vector3::new(0.0, 1.0, 0.0);

    // 4. Ray-trace *just* that “test_world”:
    let world_bbox = BVHNode::from_hittable_list(test_world);
    let sync_world: Arc<dyn Hittable + Send + Sync> = Arc::new(world_bbox);

    /*
    // Simple “probe ray” toward the cluster center:
    let origin = cam.lookfrom;
    let target = Point3::new(1.0, 352.0, 454.0);
    let dir = (target - origin).normalize();
    let test_ray = Ray::new_from(origin, dir, 0.0);
    let hit_opt = sync_world.hit(&test_ray, &Interval::new(0.001, f32::INFINITY));
    println!(">>>>> Single‐ray probe hit? {}", hit_opt.is_some());
    if let Some(rec) = hit_opt {
        println!("    Hit point = {:?},   t = {}", rec.p, rec.t);
    }
    */

    _ = cam.render(&sync_world);  // should finish in <1s at 200×200×10 SPP
    Ok(())
}