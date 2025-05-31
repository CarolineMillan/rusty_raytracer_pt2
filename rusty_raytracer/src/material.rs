use dyn_clone::DynClone;
use nalgebra::Point3;

use crate::{colour::Colour, hittable::HitRecord, ray::Ray};

pub trait Material: Send + Sync + DynClone{

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }

    fn emitted(&self, u: f32, v: f32, p: Point3<f32>) -> Colour {
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