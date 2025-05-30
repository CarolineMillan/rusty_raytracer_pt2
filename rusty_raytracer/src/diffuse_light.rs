use nalgebra::{Point3, Vector3};

use crate::{colour::Colour, hittable::HitRecord, material::Material, random_f32, ray::Ray, solid_colour::SolidColour, texture::Texture, vector_math::{reflect, refract}};

pub struct DiffuseLight {
    tex: Box<dyn Texture>,
}

impl DiffuseLight {
    pub fn new() -> Self {
        Self {
            tex: Box::new(SolidColour::new_from_colour(Colour::new())),
            //albedo: Colour::new(),
        }
    }

    pub fn new_from(albedo: Colour) -> Self {
        Self {
            tex: Box::new(SolidColour::new_from_colour(albedo)),
            //albedo,
        }
    }

    pub fn new_from_tex(tex:Box<dyn Texture>) -> Self {
        Self {
            tex,
        }
    }
    
}


impl Material for DiffuseLight {

    fn emitted(&self, u: f32, v: f32, p: Point3<f32>) -> Colour {
        return self.tex.value(u, v, &p);
    }
    
    fn clone_box(&self) -> Box<dyn Material + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for DiffuseLight {
    fn clone(&self) -> Self {
        Self {
            tex: self.tex.clone_box(),
            //albedo: self.albedo.clone(),
        }
    }
}