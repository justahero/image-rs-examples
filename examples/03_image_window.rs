/// non-working example that shows Integration is difficult sometimes
use image::{Rgb, RgbImage};
use show_image::{create_window, WindowOptions};

#[show_image::main]
fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 512;
    let height = 512;
    let mut image = RgbImage::new(width, height);
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = Rgb([0, 255, 255]));

    // image.save("test.png")?;

    // Create a window with default options, display image
    let window_options = WindowOptions::new().set_size([width, height]);
    let window = create_window("image", window_options)?;

    // show-image crate provides conversion for DynamicImage
    let image = show_image::Image::from(image);
    window.set_image("image", image)?;

    window.wait_until_destroyed()?;

    Ok(())
}
