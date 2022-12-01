use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const RESOURCE_PREFIX: &str = "resource/";

/// Read resource file as String.
pub fn read_resource(f: &str) -> String {
    // Create a path to the desired file
    let full_loc = String::from(format!("{}{}", RESOURCE_PREFIX, f));
    let path = Path::new(&full_loc);
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => (),
    }

    s
}

/// Split string on newlines. This will remove empty lines!
pub fn split_lines(content: &String) -> Vec<&str> {
    content
        .split('\n')
        .map(|x| x.trim())
        .filter(|x| x.len() > 0)
        .collect()
}

/// Split string on newlines, preserving any empty lines.
pub fn split_lines_preserve_empty(content: &String) -> Vec<&str> {
    content.split('\n').map(|x| x.trim()).collect()
}
