use crate::vec3::*;

pub struct Ray<'a> {
    origin: &'a Vec3,
    direction: &'a Vec3
}

impl Ray<'_> {
    pub fn new<'a>(origin:&'a Vec3,
               direction:&'a Vec3) -> Ray<'a> {
        Ray {
            origin,
            direction
        }
    }

    pub fn get_origin(&self) -> &Vec3 {
        return self.origin;
    }

    pub fn get_direction(&self) -> &Vec3 {
        return self.direction;
    }

    pub fn at(&self,t:f64) -> Vec3 {
        return self.origin + &(self.direction * t);
    }
}