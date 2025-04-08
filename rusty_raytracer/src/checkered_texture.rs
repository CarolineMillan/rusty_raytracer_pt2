use nalgebra::Point3;

use crate::{colour::Colour, solid_colour::SolidColour, texture::Texture};

pub struct CheckerTexture {
    inv_scale: f32,
    even: Box<dyn Texture + Send + Sync>,
    odd: Box<dyn Texture + Send + Sync>,
}

impl CheckerTexture {
    pub fn new_from_textures(scale: f32, even: Box<dyn Texture>, odd: Box<dyn Texture>) -> Self {
        Self {
            inv_scale: 1.0 / scale,
            even,
            odd,
        }
    }

    pub fn new_from_colours(scale: f32, c1: Colour, c2: Colour) -> Self {
        let col1 = Box::new(SolidColour::new_from_colour(c1));
        let col2 = Box::new(SolidColour::new_from_colour(c2));
        return Self::new_from_textures(scale, col1, col2);
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Colour {
        let x_integer = (self.inv_scale * p.x).floor() as i32;
        let y_integer = (self.inv_scale * p.y).floor() as i32;
        let z_integer = (self.inv_scale * p.z).floor() as i32;

        //println!("p: ({}, {}, {}), grid indices: ({}, {}, {})", p.x, p.y, p.z, x_integer, y_integer, z_integer);

        //println!("x: {}, y: {}, z: {}", x_integer, y_integer, z_integer);

        let is_even = (x_integer + y_integer + z_integer) % 2 == 0;
        
        return if is_even {self.even.value(u, v, p)} else {self.odd.value(u, v, p)};
    }

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for CheckerTexture {
    fn clone(&self) -> Self {
        Self {
            inv_scale: self.inv_scale.clone(),
            even: self.even.clone(),
            odd: self.odd.clone(),
        }
    }
}