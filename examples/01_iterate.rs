use image::{Rgba, RgbaImage};

fn main() {
    // create new image
    let mut image = RgbaImage::new(512, 512);

    // set pixels by iterating over width / height (classic mode)
    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(x, y, Rgba([0, 0, 255, 0]));
        }
    }
}
