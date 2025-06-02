use nalgebra::Point3;

use crate::core::colour::Colour;

pub trait Texture: Send + Sync {

    fn value(&self, u: f32, v: f32, p: &Point3<f32>) -> Colour;

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync>;
}

impl Clone for Box<dyn Texture + Send + Sync> {
    fn clone(&self) -> Box<dyn Texture + Send + Sync> {
        self.clone_box()
    }
}