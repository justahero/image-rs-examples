use image::{Rgba, RgbaImage};

fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 512;
    let height = 512;
    let mut image = RgbaImage::new(width, height);
    image
        .enumerate_pixels_mut()
        .for_each(|(x, y, mut pixel)| {
            // TODO fill in with some nice math formula / pattern
        });

    // save image to file
    image.save("04.png")?;

    Ok(())
}
