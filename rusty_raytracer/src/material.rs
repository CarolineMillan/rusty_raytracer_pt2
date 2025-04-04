use crate::{colour::Colour, hittable::HitRecord, ray::Ray};

pub trait Material: Send + Sync {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        None
    }
    fn clone_box(&self) -> Box<dyn Material + Send + Sync>;
}

impl Clone for Box<dyn Material + Send + Sync> {
    fn clone(&self) -> Box<dyn Material + Send + Sync> {
        self.clone_box()
    }
}