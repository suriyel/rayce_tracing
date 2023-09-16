use std::cmp::min;
use rand::Rng;

pub const PI:f64 = std::f64::consts::PI;
pub const INFINITY:f64 = f64::INFINITY;

pub fn degrees_to_radians(degree:f64)->f64 {
    degree * std::f64::consts::PI / 180.0
}
pub fn ff_min(a: f64, b: f64) -> f64 {
    return if a <= b { a } else { b };
}

pub fn ff_max(a: f64, b: f64) -> f64 {
    return if a >= b { a } else { b };
}

pub fn get_random_double() ->f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double(min:f64,max:f64)->f64 {
    min + (max - min) * get_random_double()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    return x;
}

#[derive(Default)]
pub struct Interval {
    pub min: f64,
    pub max: f64
}

impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Interval {
            min,
            max
        }
    }

    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    pub fn contains(&self,x:f64)->bool {
        self.min <= x && x <= self.max
    }

    pub fn expand(&self,delta:f64) -> Interval {
        let padding = delta / 2.0;
        Interval::new(self.min - padding, self.max + padding)
    }
}