use image::{Rgba, RgbaImage};
use num_complex::Complex;

const MAX_ITERATIONS: u32 = 256;

fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 512;
    let height = 512;

    let cxmin = -2f32;
    let cxmax = 1f32;
    let cymin = -1.5f32;
    let cymax = 1.5f32;

    let scalex = (cxmax - cxmin) / width as f32;
    let scaley = (cymax - cymin) / height as f32;

    let mut image = RgbaImage::new(width, height);
    image
        .enumerate_pixels_mut()
        .for_each(|(x, y, pixel)| {

            let cx = cxmin + x as f32 * scalex;
            let cy = cymin + y as f32 * scaley;

            let c = Complex::new(cx, cy);
            let mut z = Complex::new(0f32, 0f32);

            let mut i: u32 = 0;
            for t in 0..MAX_ITERATIONS {
                if z.norm() > 2.0 {
                    break;
                }
                z = z * z + c;
                i = t;
            }

            let i  = i as u8;

            *pixel = Rgba([i, i, i, 255]);
        });

    // save image to file
    image.save("04.png")?;

    Ok(())
}
