pub fn print_image(width: i32, height: i32) {
    println!("P3\n{} {}\n255", width, height);

    for j in 0..height {
        let process = format!("Scanlines remaining: {}",height-j);
        dbg!(process);
        for i in 0..width {
            let r = f64::from(i) / f64::from(width - 1);
            let g = f64::from(j) / f64::from(height - 1);
            let b = f64::from(0);

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }

    let end = String::from("Done.                 ");
    dbg!(end);
}

use std::io;
use crate::vec3::Vec3;

pub fn print_image2(width:i32,height:i32) {
    println!("P3\n{} {}\n255", width, height);
    let f_width = f64::from(width -1);
    let f_height = f64::from(height -1);
    let stdout = io::stdout();
    let mut write = stdout.lock();

    for j in 0..height {
        let process = format!("Scanlines remaining: {}",height-j);
        dbg!(process);
        for i in 0..width {
            let color = Vec3::new(
                f64::from(i) / f_width,
                f64::from(j) / f_height,
                0.0
            );
            color.write_color(&mut write)
                .expect(&format!("Failed to Write Color:{}_{}", i, j));
        }
    }

    let end = String::from("Done.                 ");
    dbg!(end);
}
