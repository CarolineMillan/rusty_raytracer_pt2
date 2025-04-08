use nalgebra::Point3;

use crate::{colour::Colour, texture::Texture};

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
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> crate::colour::Colour {
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