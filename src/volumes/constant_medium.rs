// this is a struct for a hittable constant density medium
//  ie a basic struct for fog/mist/smoke

use std::f32::INFINITY;

use crate::geometry::aabb::AABB;
use crate::core::colour::Colour;
use crate::geometry::hittable::{Hittable, HitRecord};
use crate::util::interval::Interval;
use crate::materials::isotropic::Isotropic;
use crate::core::ray::Ray;
use crate::materials::material::Material;
use crate::textures::texture::Texture;
use crate::util::vector_math::random_f32;
use nalgebra::{Vector3};

#[derive(Clone)]

pub struct ConstantMedium {
    boundary: Box<dyn Hittable>,
    neg_inv_density: f32,
    phase_function: Box<dyn Material + Send + Sync>,
}

impl ConstantMedium {

    pub fn new_from_tex(boundary: Box<dyn Hittable>, density: f32, tex: Box<dyn Texture>) -> Self {
        let neg_inv_density = -1.0/density;
        let phase_function = Box::new(Isotropic::new_from_tex(tex));
        Self {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }

    pub fn new_from_colour(boundary: Box<dyn Hittable>, density: f32, albedo: Colour) -> Self {
        let neg_inv_density = -1.0/density;
        let phase_function = Box::new(Isotropic::new_from_colour(albedo));
        Self {
            boundary,
            neg_inv_density,
            phase_function,
        }
    }

}

impl Hittable for ConstantMedium {

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if let Some(mut hit_rec1) = self.boundary.hit(ray, &Interval::universe()) {
            if let Some(mut hit_rec2) = self.boundary.hit(ray, &Interval::new(hit_rec1.t+0.0001, INFINITY)) {
                if hit_rec1.t < ray_t.min {hit_rec1.t = ray_t.min;}
                if hit_rec2.t > ray_t.max {hit_rec2.t = ray_t.max;}
                
                if hit_rec1.t >= hit_rec2.t {return None;}

                if hit_rec1.t < 0.0 {hit_rec1.t = 0.0;}

                let ray_length = ray.direction().len() as f32;
                let distance_inside_boundary = (hit_rec2.t - hit_rec1.t)*ray_length;
                let hit_distance = self.neg_inv_density * random_f32().ln();

                if hit_distance > distance_inside_boundary {return None;}
                
                let mut hit_rec = HitRecord::new();
                hit_rec.t = hit_rec1.t + hit_distance/ray_length;
                hit_rec.p = ray.at(hit_rec.t);

                hit_rec.normal = Vector3::zeros(); //arbitrary
                hit_rec.front_face = true; //also arbitrary
                hit_rec.mat = self.phase_function.clone();

                return Some(hit_rec);
            }
            else {return None;}
        }
        else {return None;}
    }

    fn bounding_box(&self) -> AABB {return self.boundary.bounding_box();}

    fn clone_box(&self) -> Box<dyn Hittable> {return Box::new((*self).clone());}
}