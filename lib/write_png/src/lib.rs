use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

pub use png::ColorType;

pub fn write_png(
    filename: &str,
    width: usize,
    height: usize,
    colortype: png::ColorType,
    data: Vec<u8>,
) -> std::io::Result<()> {
    let path = Path::new(&filename);
    let file = File::create(path)?;

    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width as u32, height as u32);

    encoder.set_color(colortype);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header()?;
    writer.write_image_data(&data)?;

    Ok(())
}
