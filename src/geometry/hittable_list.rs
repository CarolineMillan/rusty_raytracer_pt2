use crate::geometry::aabb::AABB;
use crate::geometry::hittable::{Hittable, HitRecord};
use crate::util::interval::Interval;
use crate::core::ray::Ray;

#[derive(Clone)]
pub struct HittableList {
    // Read more about Boxes, and shared pointers in cpp
    pub objects: Vec<Box<dyn Hittable>>,
    bbox: Option<AABB>,
}

impl HittableList {

    pub fn new() -> Self {
        // creates a new empty list of objects
        Self {
            objects: Vec::new(),
            bbox: None,
        }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        // adds an object to objects
        let obj_bbox = object.bounding_box();
        self.bbox = match self.bbox.take() {
            None => Some(obj_bbox),
            Some(current_bbox) => Some(AABB::new_from_boxes(&current_bbox, &obj_bbox)),
        };
        self.objects.push(object);
        //println!("hittable list bbox: {:?}", &self.bbox);
    }

    
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // returns a HitRecord if ray intersects an object between t_min and t_max
        //println!("hit for hittable list");
        let mut closest_so_far  = ray_t.max;
        let mut final_hit = None;

        for object in self.objects.iter() {
            if let Some(hit_record) = object.hit(ray, &Interval::new(ray_t.min, closest_so_far)) {
                closest_so_far = hit_record.t;
                final_hit = Some(hit_record);
            }
        }
        final_hit
    }

    fn bounding_box(&self) -> AABB {
        //println!("in hittable list bbox");
        return self.bbox.clone().expect("Hittable list has no objects");
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
}