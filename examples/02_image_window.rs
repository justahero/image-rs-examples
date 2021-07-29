use image::{Rgba, RgbaImage};
use show_image::{ImageInfo, ImageView, WindowOptions, create_window};

#[show_image::main]
fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 512;
    let height = 512;
    let mut image = RgbaImage::new(width, height);
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = Rgba([0, 255, 0, 255]));

    // Create a window with default options, display image
    let window = create_window("image", WindowOptions::default())?;
    let raw_bytes = &image.into_raw();
    let image_info = ImageInfo::rgba8(width, height);
    let image_view = ImageView::new(image_info, raw_bytes);
    window.set_image("hello", image_view)?;

    println!("ALL IS FINE??");

    window.wait_until_destroyed()?;

    Ok(())
}
