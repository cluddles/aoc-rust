use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const RESOURCE_PREFIX: &str = "resource/";

/// Read resource file as String.
pub fn read_resource(f: &str) -> String {
    // Create a path to the desired file
    let full_loc = format!("{}{}", RESOURCE_PREFIX, f);
    let path = Path::new(&full_loc);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    if let Err(why) = file.read_to_string(&mut s) {
        panic!("couldn't read {}: {}", display, why)
    }

    s
}

/// Convenience function to read resource for a particular day.
pub fn read_res_day(day: &str, filename: &str) -> String {
    read_resource(&format!("{}/{}", day, filename))
}

/// Split string on newlines. This will remove empty lines!
pub fn split_lines(content: &str) -> Vec<&str> {
    content
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .collect()
}

/// Split string on newlines, preserving any empty lines.
pub fn split_lines_keep_empty(content: &str) -> Vec<&str> {
    content.split('\n').map(|x| x.trim()).collect()
}
