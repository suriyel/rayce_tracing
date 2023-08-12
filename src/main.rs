mod image;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let width: i32 = args[1].parse().unwrap();
    let height = args[2].parse().unwrap();

    image::Image::print_image(width, height);
}