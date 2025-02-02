use exif::{In, Reader, Tag};
use image::DynamicImage;
use std::io::Cursor;

use crate::args::Args;

pub fn compress(data: Vec<u8>, args: &Args) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    // Read the image into memory
    // mozjpeg does not preserve EXIF data so orientation is manually read and the image is transformed as needed
    let image = image::load_from_memory(&data)?;
    let image = apply_orientation(image, get_orientation(&data));
    let (width, height) = (image.width(), image.height());
    let pixels = image.to_rgb8(); // Convert to RGB8 pixel format

    // Create a new compressor
    let mut compressor = mozjpeg::Compress::new(mozjpeg::ColorSpace::JCS_RGB);
    compressor.set_size(width as usize, height as usize);
    compressor.set_quality(args.jpg_quality as f32);
    compressor.set_progressive_mode();
    compressor.set_optimize_scans(true);
    compressor.set_optimize_coding(true);

    // Write the compressed JPEG
    let buf: Vec<u8> = vec![];
    let mut started = compressor.start_compress(buf)?;
    started.write_scanlines(&pixels)?;
    Ok(started.finish()?)
}

fn get_orientation(data: &Vec<u8>) -> u32 {
    let mut cursor = Cursor::new(data);
    if let Ok(exif_reader) = Reader::new().read_from_container(&mut cursor) {
        exif_reader
            .get_field(Tag::Orientation, In::PRIMARY)
            .and_then(|field| field.value.get_uint(0))
            .unwrap_or(1) // Default orientation is 1 (normal)
    } else {
        1
    }
}

fn apply_orientation(image: DynamicImage, orientation: u32) -> DynamicImage {
    match orientation {
        2 => image.fliph(),
        3 => image.rotate180(),
        4 => image.flipv(),
        5 => image.rotate90().fliph(),
        6 => image.rotate90(),
        7 => image.rotate270().fliph(),
        8 => image.rotate270(),
        _ => image, // Default: no transformation
    }
}
