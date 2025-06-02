// struct for moving hittable objects

use crate::geometry::aabb::AABB;
use crate::geometry::hittable::{Hittable, HitRecord};
use crate::util::interval::Interval;
use crate::core::ray::Ray;
use nalgebra::Vector3;

#[derive(Clone)]

pub struct Translate {
    object: Box<dyn Hittable>,
    offset: Vector3<f32>,
    bbox: AABB,
}

impl Translate {

    pub fn new(object: Box<dyn Hittable>, offset: Vector3<f32>) -> Self {
        let mut bbox = object.bounding_box();
        bbox.translate(&offset);
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {

    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
       
        let offset_r = Ray::new_from(ray.origin() - self.offset, ray.direction(), ray.time());

        if let Some(mut hit_rec) = self.object.clone().hit(&offset_r, ray_t) {
            hit_rec.p += self.offset; 
            return Some(hit_rec);
        }
        else {return None;}
    }

    fn bounding_box(&self) -> AABB {return self.bbox.clone();}

    fn clone_box(&self) -> Box<dyn Hittable> {return Box::new((*self).clone());}
}