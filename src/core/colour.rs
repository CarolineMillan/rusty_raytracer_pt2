use nalgebra::Vector3;
use std::fs::File;
use std::io::Write;

use crate::util::interval::Interval;

#[derive(Clone)]
#[derive(Debug)]
// not a new struct, we want a Newtype pattern, essentially a wrapper for Vector3<f32>
pub struct Colour(pub Vector3<f32>);

impl Colour {
    pub fn new() -> Self {
            Colour(Vector3::zeros())
    }

    pub fn new_from(r: f32, g: f32, b: f32) -> Self {
            Colour(Vector3::new(r, g, b))
    }

    /// Returns the red component.
    pub fn r(&self) -> f32 {
        self.0.x
    }
    
    /// Returns the green component.
    pub fn g(&self) -> f32 {
        self.0.y
    }
    
    /// Returns the blue component.
    pub fn b(&self) -> f32 {
        self.0.z
    }
}

pub fn write_colour(mut file: &File, pixel_colour: Colour) -> Result<(), Box<dyn std::error::Error>> {
    // write one line of pixel data to file

    let intensity = Interval::new(0.000, 0.999);
    let r_byte = (256.0*intensity.clamp(linear_to_gamma(pixel_colour.r()))) as i32;
    let g_byte = (256.0*intensity.clamp(linear_to_gamma(pixel_colour.g()))) as i32;
    let b_byte = (256.0*intensity.clamp(linear_to_gamma(pixel_colour.b()))) as i32;

    let pixel_data = format!("{} {} {}\n", r_byte, g_byte, b_byte);
    // performance note: if the project gets bigger, store pixel_data in a vector and print everything at the end
               
    // if I'm using as_bytes, I'm putting it into the wrong format initally
    // just put it in the right format to start with
    file.write_all(pixel_data.as_bytes())?;

    Ok(())
}

pub fn write_colour_string(pixel_colour: Colour) -> String {
    // write one line of pixel data to file

    let intensity = Interval::new(0.000, 0.999);
    let r_byte = (256.0*intensity.clamp(linear_to_gamma(pixel_colour.r()))) as i32;
    let g_byte = (256.0*intensity.clamp(linear_to_gamma(pixel_colour.g()))) as i32;
    let b_byte = (256.0*intensity.clamp(linear_to_gamma(pixel_colour.b()))) as i32;

    let pixel_data = format!("{} {} {}\n", r_byte, g_byte, b_byte);
    pixel_data
    // performance note: if the project gets bigger, store pixel_data in a vector and print everything at the end
               
    // if I'm using as_bytes, I'm putting it into the wrong format initally
    // just put it in the right format to start with
    //file.write_all(pixel_data.as_bytes())?;

    //Ok(())
}

pub fn linear_to_gamma(linear_component: f32) -> f32 {
    if linear_component > 0.0 {return linear_component.sqrt()} else {return 0.0;}
}

/*

This stuff lets you access Vector3<f32> directly, not sure I need/want this for now at least

// Optionally, implement Deref to make accessing Vector3's methods more ergonomic.
impl Deref for Colour {
    type Target = Vector3<f32>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Colour {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

*/