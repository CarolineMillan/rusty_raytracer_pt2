use crate::lambertian::Lambertian;
use crate::aabb::AABB;
use crate::{material::Material, ray::Ray};
use crate::interval::Interval;

use nalgebra::{Point3, Vector3};

pub trait Hittable: Send + Sync {
    // a trait will be used as a sort of "parent class" for hittable objects
    
    fn hit(&self, _ray: &Ray, _ray_t: &Interval) -> Option<HitRecord> {
        None
    }
    fn bounding_box(&self) -> AABB;

    fn clone_box(&self) -> Box<dyn Hittable>;
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Box<dyn Hittable> {
        self.clone_box()
    }
}

//#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3<f32>,
    pub normal: Vector3<f32>,
    pub mat: Box<dyn Material>,
    pub t: f32,
    pub front_face: bool,
    // texture (spherical) coords
    pub u: f32,
    pub v: f32,
}

impl HitRecord {

    // rename to default?
    
    pub fn new() -> Self {
        Self {
            p: Point3::origin(),
            normal: Vector3::zeros(),
            mat: Box::new(Lambertian::new()), //use default lambertian material
            t: 0.0,
            front_face: false, //FIXME
            u: 0.0,
            v: 0.0
        }
    }
    

    pub fn new_from(p: Point3<f32>, normal: Vector3<f32>,mat: Box<dyn Material>, t: f32) -> Self {
        Self {
            p,
            normal,
            mat,
            t,
            front_face: false, // FIXME
            u: 0.0,
            v: 0.0,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vector3<f32>) {
        // set normal vector


        self.front_face = ray.direction().dot(&outward_normal) < 0.0;
        if self.front_face {
            self.normal = outward_normal.clone()
        } 
        else {self.normal = -outward_normal};

    }
}