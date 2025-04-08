use nalgebra::{Point3, Vector3};

#[derive(Debug)]
#[derive(Clone)]

pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
    time: f32,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::origin(),
            direction: Vector3::zeros(),
            time: 0.0,
        }
    }

    pub fn new_from(origin: Point3<f32>, direction: Vector3<f32>, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(&self) -> Point3<f32> {
        self.origin
    }

    pub fn direction(&self) -> Vector3<f32> {
        self.direction
    }

    pub fn at(&self, t: f32) -> Point3<f32> {
        self.origin + t*self.direction
    }
    
    pub fn time(&self) -> f32{
        return self.time;
    }

}

