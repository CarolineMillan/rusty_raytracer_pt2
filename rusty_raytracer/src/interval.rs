use std::f32;
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    /*
    pub fn default() -> Self {
        Self {
            min: f32::INFINITY,
            max: -1.0*f32::INFINITY,
        }
    }
    */
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
        }
    }
    /*

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        (self.min <= x) && (x <= self.max)
    }
    */

    pub fn surrounds(&self, x: f32) -> bool {
        (self.min < x) && (x < self.max)
    }
    
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {return self.min}
        if x > self.max {return self.max}
        x
    }
}

//const EMPTY: Interval = Interval::new(f32::INFINITY, f32::INFINITY);