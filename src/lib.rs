mod image;
mod vec3;
mod ray;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        image::print_image(200, 100);
    }
}
