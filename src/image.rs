use std::rc::Rc;
use crate::camera::Camera;
use crate::common::{get_random_double, PI, random_double};
use crate::material::{Dielectric, Lambertian, Metal};
use crate::vec3::{Color, Point, Vec3};
use crate::sphere::*;

pub fn print_image(width:i32) {
    // sphere
    let mut world = HittableList::new();
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -1000.0, 0.0), 1000.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = get_random_double();
            let center = Point::new(get_random_double() * 0.9 + f64::from(a), 0.2, get_random_double() * 0.9 + f64::from(b));

            if (center - Point::new(4.0,0.2,0.0)).length() > 0.9 {

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random() * Color::random();
                    let center2 = center + Vec3::new(0.0, random_double(0.0, 0.5), 0.0);
                    world.add(Box::new(Sphere::new_moving_sphere(center,center2, 0.2, Rc::new(Lambertian::new(albedo)))));
                }
                else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_in_range(0.5, 1.0);
                    let fuzz = random_double(0.0, 0.5);
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(Metal::new(albedo, fuzz)))));
                }
                else {
                    // glass
                    world.add(Box::new(Sphere::new(center, 0.2, Rc::new(Dielectric::new(1.5)))));
                }
            }
        }
    }

    world.add(Box::new(Sphere::new(Point::new(0.0, 1.0, 0.0), 1.0,
                                   Rc::new(Dielectric::new(1.5)))));
    world.add(Box::new(Sphere::new(Point::new(-4.0, 1.0, 0.0), 1.0,
                                   Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))))));
    world.add(Box::new(Sphere::new(Point::new(4.0, 1.0, 0.0), 1.0,
                                   Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)))));

    let camera = Camera::new(width, 20.0, 16.0 / 9.0, 100,
                             Point::new(13.0, 2.0, 3.0),
                             Point::new(0.0, 0.0, 0.0),
                             Vec3::new(0.0, 1.0, 0.0),
                             10.0,
                             0.6);
    camera.render(&world);
}
