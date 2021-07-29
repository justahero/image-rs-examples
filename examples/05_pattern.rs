use image::{Rgba, RgbaImage};

const MAX_ITERATIONS: u32 = 256;

fn main() -> anyhow::Result<()> {
    // create new image, set pixels by iterating over width / height
    let width = 1024;
    let height = 1024;

    let cx: f32 = -0.7;
    let cy: f32 = 0.27015;
    let move_x = 0.0;
    let move_y = 0.0;
    let zoom: f32 = 1.0;

    let mut image = RgbaImage::new(width, height);

    // algorithm is based on: https://www.geeksforgeeks.org/julia-fractal-python/
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let mut zx = 1.5 * (x as f32 - width as f32 / 2.0) / (0.5 * zoom * width as f32) + move_x;
        let mut zy = 1.0 * (y as f32 - height as f32 / 2.0) / (0.5 * zoom * height as f32) + move_y;

        let mut iteration  = MAX_ITERATIONS;
        while (zx * zx + zy * zy) < 4.0 && iteration > 1 {
            let temp = zx * zx - zy * zy + cx;
            zy = 2.0 * zx * zy + cy;
            zx = temp;

            iteration -= 1;
        }

        let r = ((iteration << 16) >> 16 & 0xff) as u8;
        let g = ((iteration << 8) >> 8 & 0xff) as u8;
        let b = ((iteration) & 0xff) as u8;

        *pixel = Rgba([r, g, b, 255]);
    };

    image.save("05.png")?;

    Ok(())
}
