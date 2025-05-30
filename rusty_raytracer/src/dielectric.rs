use nalgebra::Vector3;

use crate::{colour::Colour, hittable::HitRecord, material::Material, random_f32, ray::Ray, vector_math::{reflect, refract}};

pub struct Dielectric {
    refraction_index: f32,
}

impl Dielectric {
    pub fn default() -> Self {
        Self {
            refraction_index: 0.0,
        }
    }

    pub fn new_from(refraction_index: f32) -> Self {
        Self {
            refraction_index,
        }
    }

    fn reflectance(&self, cosine: f32, ri: f32) -> f32 {
        // Schlick's approximation
        let mut r0 = (1.0-ri)/(1.0+ri);
        r0 = r0 *r0;
        r0 + (1.0-r0)*f32::powf(1.0-cosine, 5.0)
    }
    
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Colour, Ray)> {
        
        let attenuation = Colour::new_from(1.0, 1.0, 1.0);
        let ri;
        if rec.front_face {
            ri = 1.0/self.refraction_index;
        }
        else {
            ri = self.refraction_index;
        }

        let unit_direction = r_in.direction().normalize();
        
        let cos_theta = f32::min(-unit_direction.dot(&rec.normal), 1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        // snells law
        let cannot_refract = ri * sin_theta > 1.0;
        let direction: Vector3<f32>;

        if cannot_refract || (self.reflectance(cos_theta, ri) > random_f32()) {direction = reflect(&unit_direction, &rec.normal)}
        else {direction = refract(&unit_direction, &rec.normal, ri)}        

        let scattered = Ray::new_from(rec.p, direction, r_in.time());
        //println!("dielectric scattered: {:?}", scattered);
        Some((attenuation, scattered))
    }
    fn clone_box(&self) -> Box<dyn Material + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for Dielectric {
    fn clone(&self) -> Self {
        Self {
            refraction_index: self.refraction_index.clone(),
        }
    }
}