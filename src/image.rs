use std::fmt::format;
use std::io;
use std::io::BufRead;

pub struct Image {
    width: i32,
    height: i32
}

impl Image {
    pub fn print_image(width: i32, height: i32) {
        println!("P3\n{} {}\n255", width, height);

        for j in 0..height {
            let process = format!("Scanlines remaining: {}",height-j);
            dbg!(process);
            for i in 0..width {
                let r = f64::from(i) / f64::from((width - 1));
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
}
