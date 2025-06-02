use crate::{core::colour::Colour, geometry::hittable::HitRecord, materials::material::Material, core::ray::Ray, textures::solid_colour::SolidColour, textures::texture::Texture, util::vector_math::random_unit_vector};

pub struct Isotropic {
    tex: Box<dyn Texture>,
}

impl Isotropic {
    pub fn new() -> Self {
        Self {
            tex: Box::new(SolidColour::new_from_colour(Colour::new())),
            //albedo: Colour::new(),
        }
    }

    pub fn new_from_colour(albedo: Colour) -> Self {
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


impl Material for Isotropic {

    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        //let mut scatter_direction = rec.normal + random_unit_vector();
        
        //if near_zero(scatter_direction) {scatter_direction = rec.normal}// + random_unit_vector()}
        
        let scattered = Ray::new_from(rec.p, random_unit_vector(), r_in.time());
        let attenuation = self.tex.value(rec.u, rec.v, &rec.p);//self.albedo.clone();
        //println!("lambertian scattered: {:?}", scattered);
        Some((attenuation, scattered))
    }
    
    fn clone_box(&self) -> Box<dyn Material + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Isotropic {
    fn clone(&self) -> Self {
        Self {
            tex: self.tex.clone_box(),
        }
    }
}