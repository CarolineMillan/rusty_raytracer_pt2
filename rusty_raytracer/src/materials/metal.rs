use crate::{core::colour::Colour, geometry::hittable::HitRecord, materials::material::Material, core::ray::Ray};
use crate::util::vector_math::{ random_unit_vector, reflect};


pub struct Metal {
    albedo: Colour,
    fuzz: f32,
}

impl Metal {
    pub fn new() -> Self {
        Self {
            albedo: Colour::new(),
            fuzz: 0.0,
        }
    }

    pub fn new_from(albedo: Colour, mut fuzz: f32) -> Self {
        if fuzz > 1.0 {fuzz = 1.0}
        Self {
            albedo,
            fuzz,
        }
    }
    
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        
        let mut reflected = reflect(&r_in.direction(), &rec.normal);
        reflected = reflected.normalize() + (self.fuzz*random_unit_vector());

        let scattered = Ray::new_from(rec.p, reflected, r_in.time());
        let attenuation = self.albedo.clone();
        if scattered.direction().dot(&rec.normal) > 0.0 {
            //println!("metal scattered: {:?}", scattered);
            Some((attenuation, scattered))
        } else {
            None
        }
    }
    fn clone_box(&self) -> Box<dyn Material + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Metal {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo.clone(),
            fuzz: self.fuzz.clone(),
        }
    }
}