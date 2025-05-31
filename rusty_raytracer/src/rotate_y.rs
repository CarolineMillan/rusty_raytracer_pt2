// struct for moving hittable objects

use std::f32::INFINITY;

use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::vector_math::degrees_to_radians;
use nalgebra::{Point3, Vector3};

#[derive(Clone)]

pub struct RotateY {
    object: Box<dyn Hittable>,
    sin_theta: f32, 
    cos_theta: f32,
    bbox: AABB,
}

impl RotateY {

    pub fn new(object: Box<dyn Hittable>, angle: f32) -> Self {
        
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = object.bounding_box();

        let mut min = Point3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY);
        let mut max = Point3::new(-1.0*f32::INFINITY, -1.0*f32::INFINITY, -1.0*f32::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = (i as f32)*bbox.x.max + (1.0-i as f32)*bbox.x.min;
                    let y = (j as f32)*bbox.y.max + (1.0-j as f32)*bbox.y.min;
                    let z = (k as f32)*bbox.z.max + (1.0-k as f32)*bbox.z.min;

                    let newx = cos_theta*x + sin_theta*z;
                    let newz = -sin_theta*x + cos_theta*z;

                    let tester = Vector3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        bbox = AABB::new_from_extrema(min, max);

        println!("bbox_rotated: {:?}", bbox);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
       
        // transform the ray from world space to object space

        let origin = Point3::new((self.cos_theta*ray.origin().x) - (self.sin_theta*ray.origin().z), ray.origin().y, (self.sin_theta*ray.origin().x) + (self.cos_theta*ray.origin().z));
        let direction = Vector3::new((self.cos_theta*ray.direction().x) - (self.sin_theta*ray.direction().z), ray.direction().y, (self.sin_theta*ray.direction().x) + (self.cos_theta*ray.direction().z));

        let rotated_r = Ray::new_from(origin, direction, ray.time());
        
        // is there an intersection in object space?
        if let Some(mut hit_rec) = self.object.clone().hit(&rotated_r, ray_t) {
            
            // transform the intersection from object space back to world space

            hit_rec.p = Point3::new((self.cos_theta*hit_rec.p.x) + (self.sin_theta*hit_rec.p.z), hit_rec.p.y, (-1.0*self.sin_theta*hit_rec.p.x) + (self.cos_theta*hit_rec.p.z));

            hit_rec.normal = Vector3::new((self.cos_theta*hit_rec.normal.x) + (self.sin_theta*hit_rec.normal.z), hit_rec.normal.y, (-1.0*self.sin_theta*hit_rec.normal.x) + (self.cos_theta*hit_rec.normal.z));
            //hit_rec.p += self.offset; //hit_rec.set_face_normal(ray, &self.object.normal);
            return Some(hit_rec);
        }
        else {
            return None;
        }
    }

    fn bounding_box(&self) -> AABB {
        //println!("in sphere bbox");
        return self.bbox.clone();
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
}