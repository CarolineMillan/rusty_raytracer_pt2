use nalgebra::{Point3};

use crate::{core::colour::Colour, materials::material::Material, textures::solid_colour::SolidColour, textures::texture::Texture};

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