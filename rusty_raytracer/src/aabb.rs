// axis aligned bounding rectangular parallelapiped (or box), AABB for short

use std::cmp::max;

use nalgebra::Point3;

use crate::{hittable::HitRecord, interval::Interval, ray::Ray};
#[derive(Clone)]
#[derive(Debug)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn empty() -> Self {
        let mut ans = Self {
            x: Interval::empty(),
            y: Interval::empty(),
            z: Interval::empty(),
        };
        ans.pad_to_minimums();
        return ans;
    }

    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        let mut ans = Self {
            x,
            y,
            z,
        };
        ans.pad_to_minimums();
        return ans;
    }

    pub fn new_from_extrema(a: Point3<f32>, b: Point3<f32>) -> Self {
        let x = if a[0] <= b[0] {Interval::new(a[0], b[0])} else {Interval::new(b[0], a[0])};
        let y = if a[1] <= b[1] {Interval::new(a[1], b[1])} else {Interval::new(b[1], a[1])};
        let z = if a[2] <= b[2] {Interval::new(a[2], b[2])} else {Interval::new(b[2], a[2])};
        Self {
            x,
            y,
            z,
        }
    }

    pub fn new_from_boxes(box0: &AABB, box1: &AABB) -> Self {
        Self {
            x: Interval::combine(&box0.x, &box1.x),
            y: Interval::combine(&box0.y, &box1.y),
            z: Interval::combine(&box0.z, &box1.z),
        }
    }

    pub fn universe() -> Self {
        Self {
            x: Interval::default(),
            y: Interval::default(),
            z: Interval::default(),
        }
    }

    pub fn axis_interval(&self, n: u32) -> &Interval {
        if n == 1 {return &self.y}
        if n == 2 {return &self.z}
        return &self.x;
    }

    pub fn longest_axis(&self) -> u32 {
        if self.x.size() > self.y.size() {
            return if self.x.size() > self.z.size() {0} else {2};
        }
        else {
            return if self.y.size() > self.z.size() {1} else {2};
        };
    }

    pub fn hit(&self, ray: &Ray, mut ray_t: Interval) -> bool { //Option<HitRecord> {
        
        //println!("in aabb hit");

        let ray_origin = ray.origin();
        let ray_dir = ray.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0/ray_dir[axis as usize];
            let t0 = (ax.min - ray_origin[axis as usize])*adinv;
            let t1 = (ax.max - ray_origin[axis as usize])*adinv;

            if t0 < t1 {
                //println!("in t0 < t1");
                if t0 > ray_t.min {ray_t.min = t0};
                if t1 < ray_t.max {ray_t.max = t1};
            }
            else {
                
                if t1 > ray_t.min {
                    //println!("in else 1");
                    ray_t.min = t1
                };
                if t0 < ray_t.max {
                    //println!("in else 0");
                    ray_t.max = t0
                };
            }

            if ray_t.max <= ray_t.min {
                //println!("in false. max: {}, min: {}", ray_t.max, ray_t.min);
                return false 
            }; //None};
        }
        //let rec = HitRecord::new();
        return true; // Some(rec);
    }

    fn pad_to_minimums(&mut self) {
        // this function adds some padding to the bounding box to avoid problems with two dimensional objects

        let delta = 0.0001;
        if self.x.size() < delta {self.x = self.x.expand(delta)};
        if self.y.size() < delta {self.y = self.y.expand(delta)};
        if self.z.size() < delta {self.z = self.z.expand(delta)};
    }
}