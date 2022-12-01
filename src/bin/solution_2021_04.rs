extern crate aoc;
use aoc::shared;

/// A Bingo board
#[derive(Debug, Clone)]
struct Board {
    /// 2D array, structured by row; access via grid\[y]\[x]
    grid: [[i32; 5]; 5],
}

/// State, consisting of CallList and Boards
#[derive(Debug, Clone)]
struct State {
    calls: Vec<u32>,
    boards: Vec<Board>,
}

/// Parse line of text of form "12 34 56 78 90" into size-5 array
fn parse_board_row(line: &str) -> [i32; 5] {
    let mut result = [0; 5];
    let vals: Vec<i32> = line
        .split(' ')
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<i32>().unwrap())
        .collect();
    for i in 0..5 {
        result[i] = vals[i];
    }
    result
}

/// Parse 5 lines
fn parse_board(lines: &[&str]) -> Board {
    let mut grid = [[0; 5]; 5];
    for i in 0..5 {
        grid[i] = parse_board_row(lines[i]);
    }
    Board { grid }
}

/// Parse input from text. First line is call list, then some number of boards
fn parse_input(content: &String) -> State {
    // Don't forget that this scrubs any empty lines...
    let lines = shared::split_lines(&content);
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

fn proc_board(call: u32, board: &mut Board) {
    for i in 0..5 {
        for j in 0..5 {
            if board.grid[i][j] == call as i32 {
                board.grid[i][j] = -1;
            }
        }
    }
}

fn proc_call(call: u32, state: &mut State) {
    for i in 0..state.boards.len() {
        proc_board(call, &mut state.boards[i]);
    }
}

fn is_winner_row(board: &Board, y: usize) -> bool {
    for i in 0..5 {
        if board.grid[y][i] != -1 {
            return false;
        }
    }
    true
}

fn is_winner_col(board: &Board, x: usize) -> bool {
    for i in 0..5 {
        if board.grid[i][x] != -1 {
            return false;
        }
    }
    true
}

fn is_winner(board: &Board) -> bool {
    for i in 0..5 {
        if is_winner_row(board, i) || is_winner_col(board, i) {
            return true;
        }
    }
    false
}

fn find_winners(state: &State) -> Vec<usize> {
    let mut result = Vec::new();
    for i in 0..state.boards.len() {
        if is_winner(&state.boards[i]) {
            result.push(i);
        }
    }
    return result;
}

fn calculate_score(call: u32, board: &Board) -> u32 {
    let mut score = 0;
    for i in 0..5 {
        for j in 0..5 {
            if board.grid[i][j] != -1 {
                score += board.grid[i][j];
            }
        }
    }
    score as u32 * call
}

fn pop_call(state: &mut State) -> u32 {
    match state.calls.pop() {
        Some(v) => v,
        None => panic!(
            "Out of calls! {} boards remaining: {:?}",
            state.boards.len(),
            state.boards
        ),
    }
}

/// First board to win: sum of unmarked numbers, multiplied by last call
fn part1(input: &State) -> u32 {
    let mut state = input.clone();
    loop {
        let call = pop_call(&mut state);
        proc_call(call, &mut state);
        let winners = find_winners(&state);
        match winners.first() {
            Some(&v) => return calculate_score(call, &state.boards[v]),
            None => {}
        }
    }
}

/// Last board to win, scored as before
fn part2(input: &State) -> u32 {
    let mut state = input.clone();
    loop {
        let call = pop_call(&mut state);
        proc_call(call, &mut state);
        let winners = find_winners(&state);
        for i in (0..winners.len()).rev() {
            if state.boards.len() > 1 {
                state.boards.remove(winners[i]);
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
        return parse_input(&shared::read_resource("2021/04/input.test"));
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
