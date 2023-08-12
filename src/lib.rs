mod image;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        image::Image::print_image(256, 256);
    }
}
