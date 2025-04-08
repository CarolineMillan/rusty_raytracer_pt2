use crate::{colour::Colour, hittable::HitRecord, material::Material, near_zero, random_unit_vector, ray::Ray};


pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new() -> Self {
        Self {
            albedo: Colour::new(),
        }
    }

    pub fn new_from(albedo: Colour) -> Self {
        Self {
            albedo,
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        let mut scatter_direction = rec.normal + random_unit_vector();
        
        if near_zero(scatter_direction) {scatter_direction = rec.normal}// + random_unit_vector()}
        
        let scattered = Ray::new_from(rec.p, scatter_direction, r_in.time());
        let attenuation = self.albedo.clone();
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
            albedo: self.albedo.clone(),
        }
    }
}