use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Set of input files to compress
    #[arg(required = true)]
    pub paths: Vec<String>,

    /// Maximum number of threads to use
    #[arg(short, long)]
    pub jobs: Option<usize>,

    /// Format of output file names
    ///
    /// Format specifiers:
    /// - %e: File extension without leading dot
    /// - %s: File stem (file name before last dot)
    /// - %%: The '%' character
    #[arg(short, long, default_value_t = String::from("compressed_%s.%e"), verbatim_doc_comment)]
    pub output_format: String,

    /// Overwrite input files with compressed outputs (short for --output-format %s.%e)
    #[arg(long)]
    pub overwrite: bool,

    /// Quality of JPEG files (1-100; 60-80 recommended)
    #[arg(long, value_parser = clap::value_parser!(u8).range(1..=100), default_value_t = 75)]
    pub jpg_quality: u8,

    /// Strip nonessential PNG chunks
    #[arg(long, value_enum, default_value_t = PngStripOptions::Safe)]
    pub png_strip: PngStripOptions,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum PngStripOptions {
    /// Don't strip headers
    None,

    /// Strip headers that don't affect rendering
    Safe,

    /// Strip all non-critical headers
    All,
}
