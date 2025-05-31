// struct for moving hittable objects

use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
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
        println!("bbox_translated: {:?}", bbox);

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
            hit_rec.p += self.offset; //hit_rec.set_face_normal(ray, &self.object.normal);
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