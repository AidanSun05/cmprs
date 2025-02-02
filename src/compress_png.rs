use oxipng::Headers;

use crate::args::{Args, PngStripOptions};

pub fn compress(data: Vec<u8>, args: &Args) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let strip = match args.png_strip {
        PngStripOptions::None => Headers::None,
        PngStripOptions::Safe => Headers::Safe,
        PngStripOptions::All => Headers::All,
    };

    let opts = oxipng::Options {
        optimize_alpha: true, // Optimize alpha channel
        strip,
        ..Default::default()
    };

    // Optimize the PNG and store the compressed result
    let compressed_data = oxipng::optimize_from_memory(&data, &opts)?;
    Ok(compressed_data)
}
