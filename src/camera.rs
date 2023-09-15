use std::cell::RefCell;
use std::env::var;
use std::fs::{File, remove_file};
use std::io::Write;
use crate::common::{degrees_to_radians, get_random_double, INFINITY};
use crate::ray::Ray;
use crate::sphere::{HitRecord, Hittable};
use crate::vec3::{Color, cross, Point, Vec3};

pub struct Camera {
    // Rendered image width
    image_width: i32,
    // Rendered image height
    image_height: i32,
    // 视角 (广角，影响viewport_height与viewport_width)
    vfov:f64,
    //  Point camera is looking from
    look_from:Point,
    // Point camera is looking at
    look_at:Point,
    // Camera-相对"up" 方向
    vup:Vec3,
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
    pixel_delta_v: Vec3,
    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,
    // 散焦角度
    defocus_angle:f64,
    // 散焦水平半径
    defocus_disk_u:Vec3,
    // 散焦垂直半径
    defocus_disk_v:Vec3,
    // 相机lookfrom点到屏幕焦距
    focus_dist:f64,
}

impl Camera {
    pub fn new(
        width: i32, vfov: f64, aspect_ratio: f64, samples_per_pixel: i32,
        look_from: Point, look_at: Point, vup: Vec3,
        focus_dist: f64, defocus_angle: f64) -> Camera {
        // Image
        let height = f64::floor(f64::from(width) / aspect_ratio) as i32;
        let height = if height < 1 { 1 } else { height };


        // Camera
        let camera_center = look_from;
        // Camera 视距 Z轴
        let theta = degrees_to_radians(vfov);
        //视窗高度
        let viewport_height = 2.0 * (theta / 2.0).tan() * focus_dist;
        //视窗宽度
        let viewport_width = viewport_height * (f64::from(width) / f64::from(height));

        // Calculate the u,v,w unit basis vectors for the camera coordinate frame.
        let w = (look_from - look_at).unit_vector();
        let u = cross(vup,w).unit_vector();
        let v = cross(w, u);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;
        let viewport_v = (-v) * viewport_height;

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / width;
        let pixel_delta_v = viewport_v / height;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = camera_center - w * focus_dist
            - viewport_u / 2 - viewport_v / 2;
        //  Q     ->△u
        //    P . . . . . .
        // ↓  . . . . . . .
        // △v . . . . . . .
        //    . . . . . . .
        //    . . . . . . .
        //    . . . . . . .
        // pixel00即是图中P点，P为视窗中心，视窗负责将3D 2k渲染画面投影到2D上
        let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

        // 计算摄像机散镜片基本向量
        let defocus_radius = focus_dist * degrees_to_radians(defocus_angle /2.0).tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_height: height,
            image_width: width,
            vfov,
            look_from,
            look_at,
            vup,
            aspect_ratio,
            samples_per_pixel,
            max_depth: 50,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            u,
            v,
            w,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            focus_dist
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

        for j in 0..self.image_height {
            println!("Scan lines remaining: {}", (self.image_height - j));
            for i in 0..self.image_width {
                let mut temp_color = Color::new(0.0, 0.0, 0.0);
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

        let ray_origin = if self.defocus_angle <= 0.0 { self.center } else { self.defocus_disk_sample() };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = get_random_double();
        return Ray::new(ray_origin, ray_direction, ray_time)
    }

    pub fn defocus_disk_sample(&self) -> Point {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p.x()) + (self.defocus_disk_v * p.y())
    }

    pub fn pixel_sample_square(&self)->Vec3 {
        let px = -0.5 + get_random_double();
        let py = -0.5 + get_random_double();
        return self.pixel_delta_u * px + self.pixel_delta_v * py;
    }

    pub fn ray_color(&self, r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            // 达到反射层数上线，返回黑色（也可返回红色看看哪里不停的散射，但每层都需要返回红色）
            return Color::new(0.0, 0.0, 0.0);
        }
        
        // 和(0,0,-1)小球求交集
        let mut temp_rec = HitRecord::new_default();
        // 防止阴影痤疮(shadow ance)，在接近t=0时会再次击中自己
        if world.hit(r,0.001,INFINITY,&mut temp_rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if temp_rec.get_material().scatter(r, &temp_rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, world, depth - 1);
            }

            return Color::new(0.0, 0.0, 0.0);
        }

        let unit_direction = r.get_direction().unit_vector();
        let a = (unit_direction.y() + 1.0) * 0.5;
        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}