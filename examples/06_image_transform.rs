use image::{imageops::overlay, GenericImageView, Rgba, RgbaImage};

const RADIUS: f32 = 0.70;
const SOFTNESS: f32 = 0.35;
const ALPHA: f32 = 0.65;

/// Linear interpolation
pub(crate) fn mix(x: f32, y: f32, a: f32) -> f32 {
    x * (1.0 - a) + y * a
}

/// Clamps the given value between min and max boundaries
pub(crate) fn clamp(left: f32, right: f32, value: f32) -> f32 {
    let min = f32::min(left, right);
    let max = f32::max(left, right);

    f32::max(min, f32::min(max, value))
}

/// Interpolation algorithm to calculate smooth step between min and max values.
/// Based on "Book Of Shaders" algorithm: https://thebookofshaders.com/glossary/?search=smoothstep
pub(crate) fn smoothstep(edge0: f32, edge1: f32, x: f32) -> f32 {
    if edge0 == edge1 {
        return 0.0;
    }

    let t = clamp((x - edge0) / (edge1 - edge0), 0.0, 1.0);
    t * t * (3.0 - 2.0 * t)
}

fn main() -> anyhow::Result<()> {
    // load image from file, assuming it exist
    let mut image = image::io::Reader::open("05.png")
        .expect("Image not found")
        .decode()?;

    // create new image for adding vignette effect
    let mut vignette_image = RgbaImage::from_fn(image.width(), image.height(), |x, y| {
        let stx = (x as f32 / image.width() as f32) - 0.5;
        let sty = (y as f32 / image.height() as f32) - 0.5;
        let st_length = (stx * stx + sty * sty).sqrt();

        let vignette = smoothstep(RADIUS, RADIUS * RADIUS - SOFTNESS, st_length);
        let value: u8 = mix(255.0, 255.0 * vignette, ALPHA) as u8;

        // FIX THIS
        Rgba([value, value, value, 0])
    });

    overlay(&mut image, &mut vignette_image, 0, 0);

    // store as jpeg
    image.save_with_format("06.jpg", image::ImageFormat::Jpeg)?;

    Ok(())
}
