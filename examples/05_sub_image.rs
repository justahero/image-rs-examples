use image::{GenericImage, GenericImageView, Rgba, RgbaImage};

fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 512;
    let height = 512;
    let mut image = RgbaImage::new(width, height);
    image
        .pixels_mut()
        .for_each(|pixel| *pixel = Rgba([0, 255, 255, 255]));

    // get mutable view on sub image
    let mut sub_image = image.sub_image(50, 50, 100, 100);
    for x in 0..sub_image.width() {
        for y in 0..sub_image.height() {
            sub_image.put_pixel(x, y, Rgba([255, 0, 0, 255]));
        }
    }

    // save image to file
    image.save("05.png")?;

    Ok(())
}
