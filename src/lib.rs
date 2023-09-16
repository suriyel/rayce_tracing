mod image;
mod vec3;
mod ray;
mod sphere;
mod common;
mod camera;
mod material;
mod aabb;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        image::print_image(800);
    }
}
