use anyhow::{anyhow, bail, Result};
use std::str::FromStr;

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
    content.split('\n').filter(|x| keep_empty || !x.is_empty()).collect()
}

/// Split string on newlines, discarding empty lines.
pub fn split_lines(content: &str) -> Vec<&str> {
    split_lines_ext(content, false)
}

/// Split string on newlines, keeping empty lines.
pub fn split_lines_keep_empty(content: &str) -> Vec<&str> {
    split_lines_ext(content, true)
}

/// Parse a value, standardising into Result
pub fn parse_str<T: FromStr>(val: &str) -> Result<T> {
    match val.trim().parse::<T>() {
        Ok(v) => Ok(v),
        Err(_) => bail!("Could not parse '{}'", val),
    }
}

/// Split text on given delim, converting tokens with parse()
///
/// Empty tokens will be ignored.
pub fn tokenize<T: FromStr>(text: &str, delim: char) -> Result<Vec<T>> {
    text.split(delim).filter(|x| !x.is_empty()).map(|x| parse_str(x)).collect()
}

/// Split the first line of given text, converting tokens with parse()
pub fn tokenize_first_line<T: FromStr>(content: &str, delim: char) -> Result<Vec<T>> {
    tokenize(split_lines(content).first().ok_or_else(|| anyhow!("No data"))?, delim)
}
