use nalgebra::Point3;

use crate::{core::colour::Colour, textures::perlin::Perlin, textures::texture::Texture};

pub struct NoiseTexture {
    noise: Perlin,
    scale: f32,
}

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {

        // fix it so that you don't need init, ow you'll always need noise to be mutable
        //let noise = Perlin::new();
        //noise.init();
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f32, _v: f32, p: &Point3<f32>) -> Colour {
        let mut my_p = p.clone();
        let new_p = self.scale*p;
        
        let n = 0.5*(1.0 + f32::sin(new_p.z + 10.0*self.noise.turbulance(&mut my_p, 7)));
        
        return Colour::new_from(1.0*n,1.0*n,1.0*n);
    }

    fn clone_box(&self) -> Box<dyn Texture + Send + Sync> {return Box::new(self.clone());}
}

impl Clone for NoiseTexture {
    fn clone(&self) -> Self {
        Self {
            noise: self.noise.clone(),
            scale: self.scale,
        }
    }
}