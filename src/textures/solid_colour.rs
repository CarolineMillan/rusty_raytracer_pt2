use nalgebra::Point3;

use crate::{core::colour::Colour, textures::texture::Texture};

pub struct SolidColour {
    albedo: Colour,
}

impl SolidColour {
    pub fn new_from_colour(albedo: Colour) -> Self {
        Self {
            albedo,
        }
    }

    pub fn new_from_rgb(r: f32, g: f32, b: f32) -> Self {
        Self {
            albedo: Colour::new_from(r, g, b),
        }
    }
}

impl Texture for SolidColour {
    fn value(&self, _u: f32, _v: f32, _p: &Point3<f32>) -> Colour {
        return self.albedo.clone();
    }

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for SolidColour {
    fn clone(&self) -> Self {
        Self {
            albedo: self.albedo.clone(),
        }
    }
}