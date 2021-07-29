use image::{Rgb, RgbImage};

fn main() {
    // create new image
    let mut image = RgbImage::new(512, 512);

    // set pixels by iterating over width / height (classic mode)
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, Rgb([0, 0, 255]));
        }
    }
    // voilá
    // now what?
}
