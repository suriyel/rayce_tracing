use std::fs::{File, remove_file};
use std::io::Write;
use crate::common::{get_random_double, INFINITY};
use crate::ray::Ray;
use crate::sphere::{HitRecord, Hittable};
use crate::vec3::Vec3;

pub struct Camera {
    // Rendered image width
    image_width: i32,
    // Rendered image height
    image_height: i32,
    // Ratio of image width over height
    aspect_ratio:f64,
    // 抽样
    samples_per_pixel:i32,
    // Camera center
    center: Vec3,
    // Location of pixel 0, 0
    pixel00_loc: Vec3,
    // Offset to pixel to the right
    pixel_delta_u: Vec3,
    // Offset to pixel below
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn new(width:i32,aspect_ratio:f64,samples_per_pixel:i32)->Camera {
        // Image
        let height = f64::floor(f64::from(width) / aspect_ratio) as i32;
        let height = if height < 1 { 1 } else { height };


        // Camera
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (f64::from(width) / f64::from(height));
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / width;
        let pixel_delta_v = viewport_v / height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2 - viewport_v / 2;
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            image_height: height,
            image_width: width,
            aspect_ratio,
            samples_per_pixel,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        // Render
        if let Err(err) = remove_file("image.ppm"){
            if err.kind() != std::io::ErrorKind::NotFound {
                println!("{:?}", err);
            }
        }
        let mut file = File::create("image.ppm")
            .expect("Failed to create image.ppm.");
        file.write_all(format!("P3\n{} {}\n255\n", self.image_width, self.image_height).as_bytes())
            .expect("Failed to Write Color.");

        for j in (0..self.image_height).rev() {
            println!("Scan lines remaining: {}", j);
            for i in 0..self.image_width {
                let mut temp_color = Vec3::new(0.0, 0.0, 0.0);
                for s in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    temp_color += self.ray_color(&r, world);
                }
                temp_color.write_color(&mut file, self.samples_per_pixel)
                    .expect(&format!("Failed to Write Color:{}_{}", i, j));
            }
        }
    }

    pub fn get_ray(&self, i: i32, j: i32) -> Ray {
        // Get a randomly sampled camera ray for the pixel at location i,j.

        let pixel_center = self.pixel00_loc + (self.pixel_delta_u * i) + (self.pixel_delta_v * j);
        let pixel_sample = pixel_center + self.pixel_sample_square();

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        return Ray::new(ray_origin, ray_direction)
    }

    pub fn pixel_sample_square(&self)->Vec3 {
        let px = -0.5 + get_random_double();
        let py = -0.5 + get_random_double();
        return self.pixel_delta_u * px + self.pixel_delta_v * py;
    }

    pub fn ray_color(&self, r: &Ray, world: &dyn Hittable) -> Vec3 {
        // 和(0,0,-1)小球求交集
        let mut temp_rec = HitRecord::new_default();
        if world.hit(r,0.0,INFINITY,& mut temp_rec) {
            return (temp_rec.get_normal() + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
        }

        let unit_direction = r.get_direction().unit_vector();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}