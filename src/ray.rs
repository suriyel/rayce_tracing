use crate::vec3::*;

#[derive(Default)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
    tm:f64
}

impl Ray {
    pub fn new(origin: Vec3,
               direction:Vec3,
               time: f64) -> Ray {
        Ray {
            origin,
            direction,
            tm: time
        }
    }

    pub fn copy(&mut self, other: Ray) {
        self.origin = other.origin;
        self.direction = other.direction;
    }

    pub fn get_origin(&self) -> Vec3 {
        return self.origin;
    }

    pub fn get_direction(&self) -> Vec3 {
        return self.direction;
    }

    pub fn get_time(&self)->f64 {
        self.tm
    }

    pub fn at(&self, t: f64) -> Vec3 {
        return self.origin + self.direction * t;
    }
}