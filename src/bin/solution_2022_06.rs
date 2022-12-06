extern crate aoc;

use aoc::shared;

const DAY: &str = "2022/05";

/// True if the given slice contains any duplicate values
fn contains_duplicates(text: &[u8]) -> bool {
    for i in 0..text.len() {
        for j in (i+1)..text.len() {
            if text[i]==text[j] { return true; }
        }
    }
    false
}

/// Returns end position of first non-duplicate segment of given size
fn find_unique_marker(text: &[u8], len: usize) -> usize {
    (0..text.len()-len).find(|&i| !contains_duplicates(&text[i..i+len])).unwrap() + len
}

/// Unique segment, length 4
fn part1(text: &[u8]) -> usize {
    find_unique_marker(text, 4)
}

/// Unique segment, length 14
fn part2(text: &[u8]) -> usize {
    find_unique_marker(text, 14)
}

fn main() {
    let input = shared::input_as_u8(DAY, "input");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_to_u8(text: &str) -> Vec<u8> {
        text.chars().map(|x| x as u8).collect()
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&str_to_u8("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 7);
        assert_eq!(part1(&str_to_u8("bvwbjplbgvbhsrlpgdmjqwftvncz")), 5);
        assert_eq!(part1(&str_to_u8("nppdvjthqldpwncqszvftbrmjlhg")), 6);
        assert_eq!(part1(&str_to_u8("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 10);
        assert_eq!(part1(&str_to_u8("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 11);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&str_to_u8("mjqjpqmgbljsphdztnvjfqwrcgsmlb")), 19);
        assert_eq!(part2(&str_to_u8("bvwbjplbgvbhsrlpgdmjqwftvncz")), 23);
        assert_eq!(part2(&str_to_u8("nppdvjthqldpwncqszvftbrmjlhg")), 23);
        assert_eq!(part2(&str_to_u8("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")), 29);
        assert_eq!(part2(&str_to_u8("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")), 26);
    }

}
