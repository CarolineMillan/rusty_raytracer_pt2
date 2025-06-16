use dyn_clone::DynClone;
use nalgebra::Point3;

use crate::{core::colour::Colour, geometry::hittable::HitRecord, core::ray::Ray};

pub trait Material: Send + Sync + DynClone{

    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }

    fn emitted(&self, _u: f32, _v: f32, _p: Point3<f32>) -> Colour {
        return Colour::new();
    }
    fn clone_box(&self) -> Box<dyn Material + Send + Sync>;
}

dyn_clone::clone_trait_object!(Material);
/*
impl Clone for Box<dyn Material + Send + Sync> {
    fn clone(&self) -> Box<dyn Material + Send + Sync> {
        self.clone_box()
    }
}
*/