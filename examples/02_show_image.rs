use image::{Rgba, RgbaImage};
use show_image::{ImageView, ImageInfo, create_window};

#[show_image::main]
fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let mut image = RgbaImage::new(512, 512);
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = Rgba([0, 255, 0, 0]));

    Ok(())
}
