use std::cmp::Ordering;

use crate::{aabb::AABB, hittable::{HitRecord, Hittable}, hittable_list::HittableList, interval::Interval, vector_math::random_u32_within, ray::Ray};

#[derive(Clone)]
pub struct BVHNode {
    left: Box<dyn Hittable>,
    right: Box<dyn Hittable>, 
    bbox: AABB,
}

impl BVHNode {
    pub fn from_hittable_list(mut list: HittableList) -> Self {
        let len = list.objects.len();
        Self::build(&mut list.objects, 0, len)
    }

    pub fn build(objects: &mut [Box<dyn Hittable>], start: usize, end: usize) -> Self {
        
        //println!("in build");
        //let axis = random_u32_within(0, 2);

        let mut bbox = AABB::empty();

        for object_index in start..end {
            bbox = AABB::new_from_boxes(&bbox, &objects[object_index].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = |a: &Box<dyn Hittable>, b: &Box<dyn Hittable>| -> Ordering {
            let a_bbox = a.bounding_box();
            let b_bbox = b.bounding_box();
            let a_interval = a_bbox.axis_interval(axis);
            let b_interval = b_bbox.axis_interval(axis);
            // Using partial_cmp because f32 comparison can be partial.
            a_interval
                .min
                .partial_cmp(&b_interval.min)
                .unwrap_or(Ordering::Equal)
        };
        let object_span = end - start;

        //println!("after comparator");

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            (objects[start].clone(), objects[start+1].clone())
        } else {
            objects[start..end].sort_by(comparator);
            let mid = start + object_span/2;
            let left = Box::new(BVHNode::build(objects, start, mid));
            let right = Box::new(BVHNode::build(objects,mid, end));
            (left as Box<dyn Hittable>, right as Box<dyn Hittable>)
        };
        //let bbox = AABB::new_from_boxes(&left.bounding_box(), &right.bounding_box());
        
        //println!("almost there...");
        println!("Bounding Box Created: {:?}", bbox);
        Self {
            left, 
            right, 
            bbox
        }
    }

    
}

impl Hittable for BVHNode {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        //None

        //println!("hit for BVHNode");
        //println!("Checking BVHNode hit at bbox: {:?}", self.bbox);

        //println!("Ray: origin={:?}, direction={:?}, interval={:?}", ray.origin(), ray.direction(), ray_t);
        //println!("Bounding Box: {:?}", self.bbox);

        if !self.bbox.hit(ray, ray_t.clone()) {
            //println!("early exit...");
            return None;
        }

        let hit_left = self.left.hit(ray, ray_t);
        
        let max = if hit_left.is_some() {hit_left.as_ref().unwrap().t.clone()} else {ray_t.max};
        let hit_right = self.right.hit(ray, &Interval::new(ray_t.min, max));
    

        // whiich one to return if both are hits?
        if hit_right.is_some() {
            //println!("right child");
            return hit_right
        }
        else if hit_left.is_some() {
            //println!("left child");
            return hit_left
        }
         else {
            //println!("no child");
            return None
        }
        //return hit_left || hit_right;
    }

    fn bounding_box(&self) -> AABB {
        //println!("in bvh bbox");
        return self.bbox.clone();
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
}