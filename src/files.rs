use glob::glob;

pub fn format_size(size: usize) -> (f64, String) {
    // Convert a size in bytes to a formatted size with prefix
    if size >= 1000000000 {
        (size as f64 / 1000000000.0, String::from("G"))
    } else if size >= 1000000 {
        (size as f64 / 1000000.0, String::from("M"))
    } else if size >= 1000 {
        (size as f64 / 1000.0, String::from("k"))
    } else {
        (size as f64, String::from(""))
    }
}

pub fn get_glob(pattern: &str) -> Vec<String> {
    glob(pattern)
        .expect("Failed to read glob pattern")
        .map(|e| e.unwrap().to_str().expect("Invalid path").to_string())
        .collect::<Vec<_>>()
}
