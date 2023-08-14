use crate::vec3::*;

pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3,
               direction: Vec3) -> Ray{
        Ray {
            origin,
            direction
        }
    }

    pub fn get_origin(&self) -> &Vec3 {
        return &self.origin;
    }

    pub fn get_direction(&self) -> &Vec3 {
        return &self.direction;
    }

    pub fn at(&self,t:f64) -> Vec3 {
        return &self.origin + &(&self.direction * t);
    }
}