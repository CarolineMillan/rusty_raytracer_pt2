use crate::{colour::Colour, hittable::HitRecord, material::Material, vector_math::{near_zero, random_unit_vector}, ray::Ray, solid_colour::SolidColour, texture::Texture};


pub struct Lambertian {
    tex: Box<dyn Texture>,
}

impl Lambertian {
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

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        if near_zero(scatter_direction) {scatter_direction = rec.normal}// + random_unit_vector()}
        
        let scattered = Ray::new_from(rec.p, scatter_direction, r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);//self.albedo.clone();
        //println!("lambertian scattered: {:?}", scattered);
        Some((attenuation, scattered))
    }
    fn clone_box(&self) -> Box<dyn Material + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Lambertian {
    fn clone(&self) -> Self {
        Self {
            tex: self.tex.clone_box(),
            //albedo: self.albedo.clone(),
        }
    }
}