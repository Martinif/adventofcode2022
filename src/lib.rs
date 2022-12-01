use std::fs;
use std::path::Path;

pub fn load_input<P: AsRef<Path>>(path: P) -> String {
    fs::read_to_string(path).expect("Unable to read file")
}