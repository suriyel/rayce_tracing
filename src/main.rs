mod image;
mod vec3;
mod ray;
mod sphere;
mod common;
mod camera;
mod material;
mod aabb;

use std::env;
use crate::common::Interval;
use crate::vec3::Vec3;
use crate::ray::*;

fn main() {
    f_4()
}

fn f_4() {
    let a = Vec3::new(0.0,1.0,2.0);
    let b = a.get(0usize).unwrap();
    println!("{}",b);
}

fn f_3() {
    let mut v1 = Vec3::new(1.0, 1.0, 1.0);
    let v2 = Vec3::new(1.0, 1.0, 1.0);
    v1 += v2;
    println!("{:?}", v1);
}

fn f_2() {
    let v1 = Vec3::new(0.0, 0.0, 0.0);
    let v2 = Vec3::new(1.0, 1.0, 1.0);
    let ray = Ray::new(v1, v2, 0.0);
    let v3 = ray.at(2.0);
    // println!("{:?}", v3);
    println!("{:?}", v3);
}

fn f_1(){
    let args: Vec<String> = env::args().collect();

    let width: i32 = args[1].parse().unwrap();
    //let height = args[2].parse().unwrap();

    image::print_image(width);
}