use std::error::Error;
use std::ffi::OsStr;
use std::fs::{metadata, File};
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use crate::args::Args;
use crate::{compress_jpg, compress_png, files};

fn compress(ext: &str, data: Vec<u8>, args: &Args) -> Result<Vec<u8>, Box<dyn Error>> {
    match ext {
        "jpg" => compress_jpg::compress(data, args),
        "png" => compress_png::compress(data, args),
        _ => Err("Unknown file type".into()),
    }
}

fn write(path: &str, args: &Args) -> Result<(usize, usize), Box<dyn Error>> {
    let path = Path::new(path);
    let ext = path.extension().unwrap_or(OsStr::new("")).to_string_lossy();

    // Open the input file
    let input_file = File::open(path)?;
    let orig_size = input_file.metadata()?.len() as usize;
    let mut reader = BufReader::new(input_file);
    let mut data = vec![];
    reader.read_to_end(&mut data)?;
    let result = compress(&ext, data, args)?;

    let new_size = result.len();
    if new_size < orig_size {
        let new_path = if args.overwrite {
            PathBuf::from(path)
        } else {
            let parent = path.parent().unwrap();
            let name = path.file_name().unwrap().to_string_lossy();
            parent.join(format!("compressed_{}", name))
        };

        let output_file = File::create(new_path)?;
        let mut writer = BufWriter::new(output_file);
        writer.write_all(&result)?;
    }

    Ok((orig_size, new_size))
}

pub fn compress_with_output(path: &str, orig_sum: &mut usize, new_sum: &mut usize, args: &Args) {
    // Check for directories
    if metadata(path).unwrap().is_dir() {
        println!(
            "{}: Use glob patterns to select files inside directories",
            path
        );
        return;
    }

    match write(path, args) {
        Ok((orig_size, new_size)) => {
            if new_size >= orig_size {
                // Skip writing files that have increased in size
                let diff = new_size - orig_size;
                let (formatted_size, size_prefix) = files::format_size(diff);
                println!("{}: skipped, +{} {}B", path, formatted_size, size_prefix);
            } else {
                let diff = orig_size - new_size;
                let (formatted_size, size_prefix) = files::format_size(diff);
                let saved_percent = (diff as f64 / orig_size as f64) * 100.0;
                println!(
                    "{}: saved {:.2} {}B ({:.2}%)",
                    path, formatted_size, size_prefix, saved_percent
                );

                *orig_sum += orig_size;
                *new_sum += new_size;
            }
        }
        Err(e) => println!("{}: {}", path, e.to_string()),
    }
}
