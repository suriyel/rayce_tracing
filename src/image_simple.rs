use std::fs::{File, remove_file};
use std::io::Write;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn print_image() {
    let image_width = 200;
    let image_height = 100;

    if let Err(err) = remove_file("image.ppm") {
        if err.kind() != std::io::ErrorKind::NotFound {
            println!("{:?}", err);
        }
    }
    let mut file = File::create("image.ppm")
        .expect("Failed to create image.ppm.");
    file.write_all(format!("P3\n{} {}\n255\n", image_width, image_height).as_bytes())
        .expect("Failed to Write Color.");

    // 我会把发出射线的原点从图像的左下角开始沿着xy方向做增量直至遍历全图。注意我这里并没有将射线的向量设置为单位向量
    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);

    // 我会把视点(或者说摄像机, 如果你认为它是个摄像机的话)放在(0,0,0)
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..image_height).rev() {
        println!("Scan lines remaining: {}", image_height - j);
        for i in 0..image_width {
            let u = f64::from(i) / f64::from(image_width);
            let v = f64::from(j) / f64::from(image_height);
            // 从摄像机 到 4:2图片上对应像素点
            let direction = &lower_left_corner + &(&horizontal*u) + (&vertical *v);
            let r = Ray::new(&origin,&direction);
            let color = ray_color(&r);
            color.write_color(&mut file)
                .expect(&format!("Failed to Write Color:{}_{}", i, j));
        }
    }
}

pub fn ray_color(r:& Ray)->Vec3 {
    let unit_direction = r.get_direction().unit_vector();
    let a = (unit_direction.y() + 1.0) * 0.5;
    // 从白色到蓝色
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}