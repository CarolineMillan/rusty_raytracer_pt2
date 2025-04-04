use crate::hittable::{Hittable, HitRecord};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::material::Material;
use nalgebra::Point3;


pub struct Sphere {
    center: Point3<f32>,
    radius: f32,
    mat: Box<dyn Material + Send + Sync>,
}

impl Sphere {

    pub fn new(center: Point3<f32>, radius: f32, mat: Box<dyn Material>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        // Compute the ray-sphere intersection here.

        let oc = ray.origin() - self.center;
        let a = ray.direction().norm_squared(); 
        // h = half_b     
        let h = ray.direction().dot(&oc);            // dot(direction, oc)
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None
        }
        
        let dis_sqrt = discriminant.sqrt();

        let mut root = (-h-dis_sqrt)/a;

        if !ray_t.surrounds(root) {
            root = (-h + dis_sqrt)/a;
            if !ray_t.surrounds(root) {
                return None
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p-self.center)/self.radius;

        //only returns one root...
        let mut rec = HitRecord::new_from(p, normal, self.mat.clone(), t);

        rec.set_face_normal(ray, &normal);

        Some(rec)
    }
}