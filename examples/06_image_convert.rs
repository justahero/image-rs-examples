fn main() -> anyhow::Result<()> {
    // load image from file, assuming it exist
    let image = image::io::Reader::open("05.png")?.decode()?;

    // store as jpeg
    image.save_with_format("05-copy.jpg", image::ImageFormat::Jpeg)?;

    Ok(())
}
