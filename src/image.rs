use std::fs::{File, remove_file};
use std::io::Write;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub fn print_image2(width:i32) {
    // Image
    let aspect_ratio = 16.0 / 9.0;
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
    let pixel_delta_u = (&viewport_u / width).unwrap();
    let pixel_delta_v = (&viewport_v / height).unwrap();

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = &camera_center - &Vec3::new(0.0, 0.0, focal_length)
        - (&viewport_u / 2).unwrap() - (&viewport_v / 2).unwrap();
    let pixel00_loc = viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    // Render
    if let Err(err) = remove_file("image.ppm"){
        if err.kind() != std::io::ErrorKind::NotFound {
            println!("{:?}", err);
        }
    }
    let mut file = File::create("image.ppm")
        .expect("Failed to create image.ppm.");
    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())
        .expect("Failed to Write Color.");
    for j in 0..height {
        println!("Scan lines remaining: {}", height - j);
        for i in 0..width {
            let pixel_center = &pixel00_loc + &(&pixel_delta_u * i) + (&pixel_delta_v * j);
            let ray_direction = &pixel_center - &camera_center;
            let ray = Ray::new(&camera_center, &ray_direction);

            let color = ray_color(&ray);
            color.write_color(&mut file)
                .expect(&format!("Failed to Write Color:{}_{}", i, j));
        }
    }
}

pub fn ray_color(r:& Ray)->Vec3 {
    let unit_direction = r.get_direction().unit_vector();
    let a = (unit_direction.y() + 1.0) * 0.5;
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

pub fn print_image(width:i32, height:i32) {
    let f_width = f64::from(width -1);
    let f_height = f64::from(height -1);
    if let Err(err) = remove_file("image.ppm"){
        if err.kind() != std::io::ErrorKind::NotFound {
            println!("{:?}", err);
        }
    }
    let mut file = File::create("image.ppm")
        .expect("Failed to create image.ppm.");

    file.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())
        .expect("Failed to Write Color.");
    for j in 0..height {
        let process = format!("Scan lines remaining: {}",height-j);
        dbg!(process);
        for i in 0..width {
            let color = Vec3::new(
                f64::from(i) / f_width,
                f64::from(j) / f_height,
                0.0
            );
            color.write_color(&mut file)
                .expect(&format!("Failed to Write Color:{}_{}", i, j));
        }
    }

    let end = String::from("Done.                 ");
    dbg!(end);
}
