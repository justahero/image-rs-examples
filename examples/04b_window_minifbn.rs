use image::{Rgb, RgbImage};
use minifb::{Key, Window, WindowOptions};

fn generate_image(width: u32, height: u32) -> RgbImage {
    let mut image = RgbImage::new(width as u32, height as u32);
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = Rgb([0, 255, 0]));
    image
}

fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 512;
    let height = 512;
    let image = generate_image(width, height);

    // create new window
    let mut window = Window::new(
        "Test - ESC to exit",
        width as usize,
        height as usize,
        WindowOptions::default(),
    )?;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // convert image to image buffer
    let mut image_buffer: Vec<u32> = vec![];
    image.pixels().for_each(|pixel| {
        // minifbn ignores alpha channel, uses `0RGB` format.
        let r: u32 = pixel[0] as u32;
        let g: u32 = pixel[1] as u32;
        let b: u32 = pixel[2] as u32;
        image_buffer.push((r << 16) | (g << 8) + b);
    });

    // render image into window
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&image_buffer, width as usize, height as usize)?;
    }

    Ok(())
}
