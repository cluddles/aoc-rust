extern crate aoc;
use aoc::shared;

/// A Bingo board
#[derive(Debug, Clone)]
struct Board {
    /// 2D array (ish), structured by row; access via grid\[y]\[x].
    ///
    /// Marked numbers will be replaced with -1.
    grid: Vec<Vec<i32>>,
}

/// State, consisting of a Vec of numbers being called, and some Boards
#[derive(Debug, Clone)]
struct State {
    /// Numbers to call, popping from the end
    calls: Vec<u32>,
    /// Boards participating
    boards: Vec<Board>,
}

/// Parse line of text of form "12 34 56 78 90" into Vec
fn parse_board_row(line: &str) -> Vec<i32> {
    // Filter any length 0 strings to cope with double spaces
    line.split(' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect()
}

/// Parse 5 lines
fn parse_board(lines: &[&str]) -> Board {
    Board {
        grid: (0..5).map(|i| parse_board_row(lines[i])).collect(),
    }
}

/// Parse input from text. First line is call list, then some number of boards
fn parse_input(content: &str) -> State {
    // Don't forget that this scrubs any empty lines...
    let lines = shared::split_lines(content);
    // Calls - reverse so we can pop from the end
    let calls = lines[0]
        .split(',')
        .rev()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut boards = Vec::new();
    let mut i = 1;
    while i < lines.len() {
        boards.push(parse_board(&lines[i..i + 5]));
        i += 5;
    }
    State { calls, boards }
}

/// Apply call to a specific board
fn apply_call_board(call: u32, board: &mut Board) {
    for i in 0..5 {
        for j in 0..5 {
            if board.grid[i][j] == call as i32 {
                board.grid[i][j] = -1;
            }
        }
    }
}

/// Apply call to all boards in state
fn apply_call(call: u32, state: &mut State) {
    for i in 0..state.boards.len() {
        apply_call_board(call, &mut state.boards[i]);
    }
}

fn is_row_marked(board: &Board, y: usize) -> bool {
    !(0..5).any(|i| board.grid[y][i] != -1)
}

fn is_col_marked(board: &Board, x: usize) -> bool {
    !(0..5).any(|i| board.grid[i][x] != -1)
}

fn is_winner(board: &Board) -> bool {
    (0..5).any(|i| is_row_marked(board, i) || is_col_marked(board, i))
}

/// Indexes of all boards that have won
fn find_winners(state: &State) -> Vec<usize> {
    state
        .boards
        .iter()
        .enumerate()
        .filter(|(_, val)| is_winner(val))
        .map(|(i, _)| i)
        .collect()
}

/// Calculate board score, the sum of all unmarked numbers time the last call.
fn calculate_score(call: u32, board: &Board) -> u32 {
    board
        .grid
        .iter()
        .flat_map(|i| i.iter())
        .filter(|i| **i != -1)
        .sum::<i32>() as u32 * call
}

/// Pop the next number to call from the end of the list
fn pop_call(state: &mut State) -> u32 {
    match state.calls.pop() {
        Some(v) => v,
        None => panic!("Out of calls! {:?}", state),
    }
}

/// First board to win: sum of unmarked numbers, multiplied by last call
fn part1(input: &State) -> u32 {
    let mut state = input.clone();
    loop {
        let call = pop_call(&mut state);
        apply_call(call, &mut state);
        // Same as matching on find_winners(&state).first()
        if let Some(&v) = find_winners(&state).first() {
            return calculate_score(call, &state.boards[v])
        }
    }
}

/// Last board to win, scored as before
fn part2(input: &State) -> u32 {
    let mut state = input.clone();
    loop {
        let call = pop_call(&mut state);
        apply_call(call, &mut state);
        // Iterate backwards over winners to avoid indexes getting out of whack
        for &winner in find_winners(&state).iter().rev() {
            if state.boards.len() > 1 {
                state.boards.remove(winner);
            } else {
                return calculate_score(call, &state.boards.pop().unwrap());
            }
        }
    }
}

fn main() {
    let input = parse_input(&shared::read_resource("2021/04/input"));
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_test_input() -> State {
        parse_input(&shared::read_resource("2021/04/input.test"))
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&gen_test_input()), 4512);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&gen_test_input()), 1924);
    }
}
