use std::path::{Path, PathBuf};
use std::str::FromStr;
use crate::harness::{DynResult, SimpleError};

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

pub fn resource_path() -> PathBuf {
    Path::new(MANIFEST_DIR).join("..").join("resource")
}

pub fn resource_path_day_filename(day: &str, filename: &str) -> PathBuf {
    resource_path().join(day).join(filename)
}

/// Convenience function to read resource for a particular day.
pub fn input_as_str(day: &str, filename: &str) -> String {
    std::fs::read_to_string(resource_path_day_filename(day, filename)).unwrap()
}

/// Convenience function to read resource for a particular day, as Vec of u8.
pub fn input_as_u8(day: &str, filename: &str) -> Vec<u8> {
    std::fs::read(resource_path_day_filename(day, filename)).unwrap()
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

/// Parse a value, standardising into DynResult
pub fn parse_str<T: FromStr>(val: &str) -> DynResult<T> {
    match val.trim().parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => Err(SimpleError::new_dyn(format!("Could not parse '{}'", val))),
    }
}

/// Split text on given delim, converting tokens with parse()
///
/// Empty tokens will be ignored.
pub fn tokenize<T: FromStr>(text: &str, delim: char) -> DynResult<Vec<T>>
// where
//     <T as FromStr>::Err: Error,
{
    text
        .split(delim)
        .filter(|x| !x.is_empty())
        .map(|x| parse_str(x))
        .collect()
}

/// Split the first line of given text, converting tokens with parse()
pub fn tokenize_first_line<T: FromStr>(content: &str, delim: char) -> DynResult<Vec<T>>
    // where
    //     <T as FromStr>::Err: Error,
{
    tokenize(
        split_lines(content)
            .first()
            .ok_or_else(|| SimpleError::new_dyn("No data"))?,
        delim,
    )
}
