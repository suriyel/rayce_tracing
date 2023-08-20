mod image;
mod image_simple;
mod vec3;
mod ray;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        image::print_image(800);
    }
}
