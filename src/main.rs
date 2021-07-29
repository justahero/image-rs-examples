use image::{Rgb, RgbImage};

fn main() {
    let mut image = RgbImage::new(512, 512);
    for (x, y, p) in image.enumerate_pixels_mut() {
        let r = (x % 255) as u8;
        let g = (y % 255) as u8;
        let b = ((2 * x + 2 * y) % 255) as u8;
        *p = Rgb([r, g, b]);
    }
    image.save("hello_world.png").unwrap();
}