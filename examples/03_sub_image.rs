use image::{GenericImage, GenericImageView, Rgba, RgbaImage};

fn main() -> anyhow::Result<()> {
    let mut image = RgbaImage::new(512, 512);
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = Rgba([0, 255, 128, 255]));

    // get mutable view on sub image
    let mut sub_image = image.sub_image(50, 50, 100, 100);

    // SubImage does not provide `pixels_mut()` or `enumerate_pixels_mut()`

    for x in 0..sub_image.width() {
        for y in 0..sub_image.height() {
            sub_image.put_pixel(x, y, Rgba([128, 128, 0, 255]));
        }
    }

    image.save("03.png")?;

    Ok(())
}
