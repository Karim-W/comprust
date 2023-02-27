fn main() {
    let jpeg_data = std::fs::read("examples/parrots.jpg")?;

    // decompress `jpeg_data` into an `image::RgbImage`
    let image: image::RgbImage = turbojpeg::decompress_image(&jpeg_data)?;

    // compress `image` into JPEG with quality 95 and 2x2 chrominance subsampling
    let jpeg_data = turbojpeg::compress_image(&image, 95, turbojpeg::Subsamp::Sub2x2)?;

    // save compressed JPEG to a file
    std::fs::write("examples/parrots-compressed.jpg", &jpeg_data)?;
}
