use std::fs::{File, remove_file};
use std::io::Write;
use crate::vec3::Vec3;

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
