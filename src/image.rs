use crate::camera::Camera;
use crate::vec3::Vec3;
use crate::sphere::*;

pub fn print_image(width:i32) {
    // sphere
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Vec3::new(0.0,0.0,-1.0),0.5)));
    world.add(Box::new(Sphere::new(Vec3::new(0.0,-100.5,-1.0),100.0)));

    let camera = Camera::new(width, (16.0 / 9.0), 100);
    camera.render(&world);
}
