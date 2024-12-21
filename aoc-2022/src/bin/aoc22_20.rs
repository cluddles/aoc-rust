extern crate aoc_lib;

use anyhow::Result;
use aoc_lib::common;
use aoc_lib::harness::*;

pub struct Day20;

type Input = Vec<Number>;
// Values in part 2 are getting large
type Output = i64;

impl Solution<Input, Output> for Day20 {
    fn info(&self) -> SolutionInfo {
        SolutionInfo::new("Grove Positioning System", 2022, 20)
    }

    fn parse_input(&self, resource: &dyn Resource) -> Result<Input> {
        Ok(convert_to_input(&common::tokenize(&resource.as_str()?, '\n')?))
    }

    fn solve_part1(&self, input: &Input) -> Result<Output> {
        let mut to_mix = input.clone();
        mix(&mut to_mix);
        Ok(score(&to_mix))
    }

    fn solve_part2(&self, input: &Input) -> Result<Output> {
        let mut to_mix =
            input.iter().map(|n| Number { start: n.start, val: n.val * DECRYPTION_KEY }).collect();
        for _ in 0..10 {
            mix(&mut to_mix);
        }
        Ok(score(&to_mix))
    }
}

/// Because values aren't unique, we need some way of tracking which ones are which...
#[derive(Debug, Clone)]
struct Number {
    val: i64,
    start: usize,
}

/// Munge vec of ints into our input
fn convert_to_input(vals: &[i64]) -> Input {
    vals.iter().enumerate().map(|(i, v)| Number { val: *v, start: i }).collect()
}

/// Do a single mix pass on the given vec
fn mix(vals: &mut Vec<Number>) {
    for i in 0..vals.len() {
        mix_one(i, vals);
    }
}

/// Mix a single value, based on the given start position
fn mix_one(start: usize, to_mix: &mut Vec<Number>) {
    // Find the number we're moving
    let (from, n) =
        to_mix.iter().enumerate().find(|(_, v)| v.start == start).expect("value must be present");
    // Moving 0 is a no-op
    if n.val == 0 {
        return;
    }
    // This is surprisingly fiddly
    let to = (from as i64 + n.val).rem_euclid(to_mix.len() as i64 - 1) as usize;
    let number = to_mix.remove(from);
    to_mix.insert(to, number);
}

/// Find "0", then sum values at offset 1000, 2000 and 3000 from it.
fn score(vals: &Vec<Number>) -> i64 {
    let p = vals.iter().position(|n| n.val == 0).expect("0 must be present");
    vals[(p + 1000) % vals.len()].val
        + vals[(p + 2000) % vals.len()].val
        + vals[(p + 3000) % vals.len()].val
}

/// Part 2 decryption key value
const DECRYPTION_KEY: i64 = 811589153;

fn main() -> Result<()> {
    run_solution(&Day20)
}

#[cfg(test)]
mod tests {
    use super::*;

    // I had a LOT of trouble with things not quite working, or (more aggravatingly) working for
    // all the tests and examples but not working with the real input, so loads of tests now live
    // here. Most of my issues stemmed from being a dunce and assuming the input values were
    // unique (which they were in the examples...)

    // Apply a single mix
    fn do_mix_one(val: i64, mix: &[i64]) -> Vec<i64> {
        let pos = mix.iter().position(|v| *v == val).expect("value must be present");
        do_mix_one_from(pos, mix)
    }
    // Apply a single mix, taking the element from given position
    fn do_mix_one_from(pos: usize, mix: &[i64]) -> Vec<i64> {
        let mut result = convert_to_input(mix);
        mix_one(pos, &mut result);
        result.iter().map(|x| x.val).collect()
    }
    // Check equality, bearing in mind the vecs may be rotated relative to one another
    fn rotate_eq(v1: &[i64], v2: &mut [i64]) {
        for _ in 0..v1.len() {
            if v1[0] == v2[0] {
                assert_eq!(v1, v2);
                return;
            }
            v2.rotate_right(1);
        }
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_p1_ex1() {
        rotate_eq(&do_mix_one(1, &[4, 5, 6, 1, 7, 8, 9]), &mut [4, 5, 6, 7, 1, 8, 9]);
    }
    #[test]
    fn test_p1_ex2() {
        rotate_eq(&do_mix_one(-2, &[4, -2, 5, 6, 7, 8, 9]), &mut [4, 5, 6, 7, 8, -2, 9]);
    }

    #[test]
    fn test_p1_mix_1() {
        rotate_eq(&do_mix_one(1, &[1, 2, -3, 3, -2, 0, 4]), &mut [2, 1, -3, 3, -2, 0, 4]);
    }
    #[test]
    fn test_p1_mix_2() {
        rotate_eq(&do_mix_one(2, &[2, 1, -3, 3, -2, 0, 4]), &mut [1, -3, 2, 3, -2, 0, 4]);
    }
    #[test]
    fn test_p1_mix_minus_3() {
        rotate_eq(&do_mix_one(-3, &[1, -3, 2, 3, -2, 0, 4]), &mut [1, 2, 3, -2, -3, 0, 4]);
    }
    #[test]
    fn test_p1_mix_3() {
        rotate_eq(&do_mix_one(3, &[1, 2, 3, -2, -3, 0, 4]), &mut [1, 2, -2, -3, 0, 3, 4]);
    }
    #[test]
    fn test_p1_mix_minus_2() {
        rotate_eq(&do_mix_one(-2, &[1, 2, -2, -3, 0, 3, 4]), &mut [1, 2, -3, 0, 3, 4, -2]);
    }
    #[test]
    fn test_p1_mix_0() {
        rotate_eq(&do_mix_one(0, &[1, 2, -3, 0, 3, 4, -2]), &mut [1, 2, -3, 0, 3, 4, -2]);
    }
    #[test]
    fn test_p1_mix_4() {
        rotate_eq(&do_mix_one(4, &[1, 2, -3, 0, 3, 4, -2]), &mut [1, 2, -3, 4, 0, 3, -2]);
    }

    // Wrapping behaviour was totally breaking my first implementation, so here's some extra tests
    // to make sure I'm not doing anything insane.
    #[test]
    fn test_mix_big() {
        rotate_eq(&do_mix_one(8, &[1, 2, 3, 4, 5, 8, 6]), &mut [1, 8, 2, 3, 4, 5, 6]);
    }
    #[test]
    fn test_mix_bigger() {
        rotate_eq(&do_mix_one(11, &[1, 2, 3, 4, 5, 11, 6]), &mut [1, 2, 3, 4, 11, 5, 6]);
    }
    #[test]
    fn test_mix_big_minus() {
        rotate_eq(&do_mix_one(-8, &[1, 2, 3, 4, 5, -8, 6]), &mut [1, 2, 3, -8, 4, 5, 6]);
    }
    #[test]
    fn test_mix_bigger_minus() {
        rotate_eq(&do_mix_one(-11, &[1, 2, 3, 4, 5, -11, 6]), &mut [-11, 1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_wrap() {
        rotate_eq(&do_mix_one(1, &[2, 3, 4, 1]), &mut [2, 1, 3, 4]);
    }
    #[test]
    fn test_wrap_minus() {
        rotate_eq(&do_mix_one(-1, &[-1, 2, 3, 4]), &mut [2, 3, -1, 4]);
    }

    #[test]
    fn test_mix_dupe() {
        rotate_eq(
            &do_mix_one_from(5, &[79, 0, 1, 2, 3, 79, 4, 5, 6, 7]),
            &mut [79, 0, 1, 79, 2, 3, 4, 5, 6, 7],
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(test_solution(&Day20, SolutionPart::One), 3);
    }

    #[test]
    fn test_part2() {
        assert_eq!(test_solution(&Day20, SolutionPart::Two), 1623178306);
    }
}
