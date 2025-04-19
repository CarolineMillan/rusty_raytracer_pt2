use nalgebra::Point3;

use crate::{
    colour::Colour, 
    interval::Interval, 
    rtw_image::RTWImage, 
    texture::Texture
};

pub struct ImageTexture {
    pub image: RTWImage,
}

impl ImageTexture {
    pub fn new_from_filename(filename: &str) -> Self {
        Self {
            image: RTWImage::new_from(filename).expect("failed to load image"),
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u1: f32, v1: f32, p: &Point3<f32>) -> Colour {
        if self.image.height() <= 0 {return Colour::new_from(0.0, 1.0, 1.0)};
        if u1.is_nan() || v1.is_nan() { return Colour::new_from(1.0, 0.0, 1.0)}; // hot pink to highlight issues
        
        let u = Interval::new(0.0, 1.0).clamp(u1);
        let v = 1.0 - Interval::new(0.0, 1.0).clamp(v1);
        let i = u*self.image.width() as f32;
        let j = v*self.image.height() as f32;
        let pixel = self.image.pixel_data(i as i32, j as i32);

        let colour_scale = 1.0/255.0;

        //println!("(u1, v1) = ({}, {}), (u, v) = ({}, {}), (i, j) = ({}, {}), pixel = {:?}", u1, v1, u, v, i, j, pixel);

        return Colour::new_from(colour_scale*pixel[0] as f32, colour_scale*pixel[1] as f32, colour_scale*pixel[2] as f32)
    }

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync> {
        Box::new(self.clone())
    }
}

impl Clone for ImageTexture {
    fn clone(&self) -> Self {
        Self {
            image: self.image.clone(),
        }
    }
}