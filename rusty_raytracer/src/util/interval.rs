use std::f32;
#[derive(Clone)]
#[derive(Debug)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    
    pub fn default() -> Self {
        // universe
        Self {
            min: f32::INFINITY,
            max: -1.0*f32::INFINITY,
        }
    }

    pub fn empty() -> Self {
        Self {
            min: 0.0,
            max: 0.0,
        }
    }

    pub fn universe() -> Self {
        Self {
            min: -1.0*f32::INFINITY,
            max: f32::INFINITY,
        }
    }
    
    pub fn new(min: f32, max: f32) -> Self {
        Self {
            min,
            max,
        }
    }

    pub fn combine(a: &Interval, b: &Interval) -> Self {
        Self {
            min: if a.min <= b.min {a.min} else {b.min},
            max: if a.max >= b.max {a.max} else {b.max},
        }
    }
    

    pub fn size(&self) -> f32 {
        self.max - self.min
    }

    pub fn contains(&self, x: f32) -> bool {
        (self.min <= x) && (x <= self.max)
    }
    

    pub fn surrounds(&self, x: f32) -> bool {
        (self.min < x) && (x < self.max)
    }
    
    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {return self.min}
        if x > self.max {return self.max}
        x
    }

    pub fn expand(&self, delta: f32) -> Interval {
        let padding = delta;
        return Interval::new(self.min - padding, self.max + padding);
    }

    pub fn translate(&mut self, offset: f32) {
        self.min += offset;
        self.max += offset;
    }
}

//const EMPTY: Interval = Interval::new(f32::INFINITY, f32::INFINITY);