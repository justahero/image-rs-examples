use image::{Rgba, RgbaImage};

fn main() -> anyhow::Result<()> {
    // create new image
    let width = 512;
    let height = 512;
    let mut image = RgbaImage::new(width, height);

    // use iterator `enumerate_pixels_mut`
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let r = ((x * 2) % 255) as u8;
        let g = ((x + y) % 255) as u8;
        let b = ((y * 2) % 255) as u8;

        *pixel = Rgba([r, g, b, 255]);
    }

    // save image to file
    image.save("03.png")?;

    Ok(())
}
