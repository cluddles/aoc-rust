use std::path::PathBuf;
use std::str::FromStr;

pub fn resource_path_base() -> PathBuf {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("../resource");
    d
}

pub fn resource_path(day: &str, filename: &str) -> PathBuf {
    let mut d = resource_path_base();
    d.push(format!("{}/{}", day, filename));
    d
}

/// Convenience function to read resource for a particular day.
pub fn input_as_str(day: &str, filename: &str) -> String {
    std::fs::read_to_string(resource_path(day, filename)).unwrap()
}

/// Convenience function to read resource for a particular day, as Vec of u8.
pub fn input_as_u8(day: &str, filename: &str) -> Vec<u8> {
    std::fs::read(resource_path(day, filename)).unwrap()
}

/// Convert string to Vec of u8
pub fn str_to_u8(text: &str) -> Vec<u8> {
    text.chars().map(|x| x as u8).collect()
}

/// Convert Vec of u8 to string
pub fn u8_to_str(input: &[u8]) -> String {
    input.iter().map(|&x| x as char).collect()
}

/// Split string on newlines, optionally keeping empty lines.
fn split_lines_ext(content: &str, keep_empty: bool) -> Vec<&str> {
    content
        .split('\n')
        .filter(|x| keep_empty || !x.is_empty())
        .collect()
}

/// Split string on newlines, discarding empty lines.
pub fn split_lines(content: &str) -> Vec<&str> {
    split_lines_ext(content, false)
}

/// Split string on newlines, keeping empty lines.
pub fn split_lines_keep_empty(content: &str) -> Vec<&str> {
    split_lines_ext(content, true)
}

/// Parse a value, panicking on error (without relying on Debug)
fn parse<T: FromStr>(val: &str) -> T {
    match val.trim().parse::<T>() {
        Ok(v) => v,
        Err(_) => panic!("Could not parse '{}'", val),
    }
}

/// Split text on given delim, converting tokens with parse()
///
/// Empty tokens will be ignored.
pub fn tokenize<T: FromStr>(text: &str, delim: char) -> Vec<T> {
    text.split(delim)
        .filter(|x| !x.is_empty())
        .map(parse)
        .collect()
}

/// Split the first line of given text, converting tokens with parse()
pub fn tokenize_first_line<T: FromStr>(content: &str, delim: char) -> Vec<T> {
    tokenize(split_lines(content).first().unwrap(), delim)
}
