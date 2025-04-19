use std::env::consts;

use crate::aabb::AABB;
use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;
use nalgebra::{Point3, Vector3};
use nalgebra::RealField;

#[derive(Clone)]

pub struct Sphere {
    center: Ray,
    radius: f32,
    mat: Box<dyn Material + Send + Sync>,
    bbox: AABB,
}

impl Sphere {

    pub fn new(center: Point3<f32>, radius: f32, mat: Box<dyn Material>) -> Self {
        let rvec = Vector3::new(radius, radius, radius);
        Self {
            center: Ray::new_from(center, Vector3::zeros(), 0.0),
            radius,
            mat,
            bbox: AABB::new_from_extrema(center - rvec, center + rvec),
        }
    }

    pub fn new_moving(center1: Point3<f32>, center2: Point3<f32>, radius: f32, mat: Box<dyn Material>) -> Self {
        let center = Ray::new_from(center1, center2-center1, 0.0);
        let rvec = Vector3::new(radius, radius, radius);
        let box1 = AABB::new_from_extrema(center.at(0.0) - rvec, center.at(0.0) + rvec);
        let box2 = AABB::new_from_extrema(center.at(1.0) - rvec, center.at(1.0) + rvec);
        Self {
            center,
            radius,
            mat,
            bbox: AABB::new_from_boxes(&box1, &box2),
        }
    }

    pub fn get_sphere_uv(p: &Point3<f32>) -> (f32, f32) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //     <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
        //     <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
        //     <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
        let y_clamped = p.y.max(-1.0).min(1.0);
        let theta = y_clamped.acos();
        let phi = -p.z.atan2(p.x) + std::f32::consts::PI;

        return (phi/(2.0*std::f32::consts::PI), 1.0 - (theta/std::f32::consts::PI));
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Compute the ray-sphere intersection here.
        //println!("In sphere hit");

        let current_center = self.center.at(ray.time());
        let oc = ray.origin() - current_center; // - self.center.origin();
        let a = ray.direction().norm_squared(); 
        // h = half_b     
        let h = ray.direction().dot(&oc);            // dot(direction, oc)
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        //println!("oc: {:?}", oc);
        //println!("discriminant: {}", discriminant);

        if discriminant < 0.0 {
            return None
        }
        
        let dis_sqrt = discriminant.sqrt();

        let mut root = (-h-dis_sqrt)/a;

        
        //println!("roots: {}, {}", (-h - dis_sqrt)/a, (-h + dis_sqrt)/a);

        if !ray_t.surrounds(root) {
            root = (-h + dis_sqrt)/a;
            if !ray_t.surrounds(root) {
                return None
            }
        }

        


        let t = root;
        let p = ray.at(t);
        let normal = (p-current_center)/self.radius; //(p-self.center)/self.radius;

        //only returns one root...
        let mut rec = HitRecord::new_from(p, normal, self.mat.clone(), t);

        rec.set_face_normal(ray, &normal);
        // the tutorial used &normal for getting the uv, but it's not a point?
        let unit_p = Point3::new(normal.x, normal.y, normal.z);
        (rec.u, rec.v) = Sphere::get_sphere_uv(&unit_p);

        Some(rec)
    }

    fn bounding_box(&self) -> AABB {
        //println!("in sphere bbox");
        return self.bbox.clone();
    }

    fn clone_box(&self) -> Box<dyn Hittable> {
        Box::new((*self).clone())
    }
}