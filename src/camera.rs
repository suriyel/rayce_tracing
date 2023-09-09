use std::cell::RefCell;
use std::env::var;
use std::fs::{File, remove_file};
use std::io::Write;
use crate::common::{degrees_to_radians, get_random_double, INFINITY};
use crate::ray::Ray;
use crate::sphere::{HitRecord, Hittable};
use crate::vec3::Vec3;

pub struct Camera {
    // Rendered image width
    image_width: i32,
    // Rendered image height
    image_height: i32,
    // 视角 (广角，影响viewport_height与viewport_width)
    vfov:f64,
    // Ratio of image width over height
    aspect_ratio:f64,
    // 抽样
    samples_per_pixel:i32,
    // 反射递归层数
    max_depth:i32,
    // 相机中心点(0,0,0)
    center: Vec3,
    // 视窗原点(0,0,-1),右手系规范,Y轴向上,X轴向右,摄像机看向方向为Z轴负方向
    pixel00_loc: Vec3,
    // 视窗向右(X轴)偏移单位向量，例如视窗宽度为2，width 800,单位向量为 (2/800,0,0)
    pixel_delta_u: Vec3,
    // 视窗向下(Y轴)偏移单位向量，例如视窗高度为1，height 600，单位向量为 (0,1/600,0)
    pixel_delta_v: Vec3
}

impl Camera {
    pub fn new(width: i32, vfov: f64, aspect_ratio: f64, samples_per_pixel: i32) -> Camera {
        // Image
        let height = f64::floor(f64::from(width) / aspect_ratio) as i32;
        let height = if height < 1 { 1 } else { height };


        // Camera
        let focal_length = 1.0; // Camera焦距 Z轴
        let theta = degrees_to_radians(vfov);
        let viewport_height = 2.0 * (theta/2.0).tan() * focal_length; //视窗高度
        let viewport_width = viewport_height * (f64::from(width) / f64::from(height)); //视窗宽度
        let camera_center = Vec3::new(0.0, 0.0, 0.0);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / width;
        let pixel_delta_v = viewport_v / height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length)
            - viewport_u / 2 - viewport_v / 2;
        //  Q     ->△u
        //    P . . . . . .
        // ↓  . . . . . . .
        // △v . . . . . . .
        //    . . . C . . .
        //    . . . . . . .
        //    . . . . . . .
        // pixel00即是图中C点，P为(0,0)，所以x、y、z都是负数
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        Camera {
            image_height: height,
            image_width: width,
            vfov,
            aspect_ratio,
            samples_per_pixel,
            max_depth: 50,
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
                    temp_color += self.ray_color(&r, world, self.max_depth);
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

    pub fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        if depth <= 0 {
            // 达到反射层数上线，返回黑色（也可返回红色看看哪里不停的散射，但每层都需要返回红色）
            return Vec3::new(0.0, 0.0, 0.0);
        }
        
        // 和(0,0,-1)小球求交集
        let mut temp_rec = HitRecord::new_default();
        // 防止阴影痤疮(shadow ance)，在接近t=0时会再次击中自己
        if world.hit(r,0.001,INFINITY,&mut temp_rec) {
            let mut scattered = Ray::new_default();
            let mut attenuation = Vec3::new_default();
            if temp_rec.get_material().scatter(r, &temp_rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }

            return Vec3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.get_direction().unit_vector();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }
}