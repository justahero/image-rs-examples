use image::{Rgba, RgbaImage};

fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 512;
    let height = 512;
    let mut image = RgbaImage::new(width, height);
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = Rgba([0, 255, 0, 255]));

    // save image to file
    image.save("03.png")?;

    Ok(())
}
