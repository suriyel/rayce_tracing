use std::rc::Rc;
use crate::camera::Camera;
use crate::common::PI;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::vec3::Vec3;
use crate::sphere::*;

pub fn print_image(width:i32) {
    // sphere
    let r = (PI / 4.0).cos();
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(-r, 0.0, -1.0), r,
                                   Rc::new(Lambertian::new(Vec3::new(0.0, 0.0, 1.0))))));
    world.add(Box::new(Sphere::new(Vec3::new(r, 0.0, -1.0), r,
                                   Rc::new(Lambertian::new(Vec3::new(1.0, 0.0, 0.0))))));
    // world.add(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5,
    //                                Rc::new(Lambertian::new(Vec3::new(0.7, 0.3, 0.3))))));
    // world.add(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0,
    //                                Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))))));
    // world.add(Box::new(Sphere::new(Vec3::new(1.0,0.0,-1.0),0.5,
    //                                Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.0)))));
    // world.add(Box::new(Sphere::new(Vec3::new(-1.0,0.0,-1.0),0.5,
    //                                Rc::new(Dielectric::new(1.5)))));

    let camera = Camera::new(width, 90.0, 16.0 / 9.0, 100);
    camera.render(&world);
}
