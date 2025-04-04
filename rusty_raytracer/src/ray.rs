use nalgebra::{Point3, Vector3};

pub struct Ray {
    origin: Point3<f32>,
    direction: Vector3<f32>,
}

impl Ray {
    pub fn new() -> Self {
        Self {
            origin: Point3::origin(),
            direction: Vector3::zeros(),
        }
    }

    pub fn new_from(origin: Point3<f32>, direction: Vector3<f32>) -> Self {
        Self {
            origin,
            direction,
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

}

