use crate::Direction::*;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::PartialEq;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

#[derive(PartialEq, Eq, Default, Copy, Clone, Debug, Hash)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self) {
        match self {
            Up => *self = Right,
            Right => *self = Down,
            Down => *self = Left,
            Left => *self = Up,
        }
    }
}

fn guard_exits_grid(
    guard_x: usize,
    guard_y: usize,
    grid_x: usize,
    grid_y: usize,
    dir: Direction,
) -> bool {
    guard_y == grid_y - 1 && dir == Down
        || guard_y == 0 && dir == Up
        || guard_x == grid_x - 1 && dir == Right
        || guard_x == 0 && dir == Left
}

fn move_guard(grid: &[Vec<char>], guard_x: &mut usize, guard_y: &mut usize, dir: &mut Direction) {
    match dir {
        Up => {
            if grid[*guard_y - 1][*guard_x] == '#' {
                dir.turn();
                // move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_y -= 1
            }
        }
        Right => {
            if grid[*guard_y][*guard_x + 1] == '#' {
                dir.turn();
                // move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_x += 1
            }
        }
        Down => {
            if grid[*guard_y + 1][*guard_x] == '#' {
                dir.turn();
                // move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_y += 1
            }
        }
        Left => {
            if grid[*guard_y][*guard_x - 1] == '#' {
                dir.turn();
                // move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_x -= 1
            }
        }
    };
}

fn simulate_with_obstacle(
    grid: &[Vec<char>],
    obstacle_pos: (usize, usize),
    mut pos: (usize, usize),
    rows: usize,
    cols: usize,
) -> bool {
    let mut dir = Up;
    let mut visited: HashSet<((usize, usize), Direction)> = HashSet::new();
    let max_steps = rows * cols * 4;

    let mut steps = 0;

    while steps < max_steps {
        let state = (pos, dir);
        if visited.contains(&state) {
            return true;
        }
        visited.insert(state);
        let (row, col) = pos;
        let (next_row, next_col) = match dir {
            Up => (row.wrapping_sub(1), col),
            Right => (row, col + 1),
            Down => (row + 1, col),
            Left => (row, col.wrapping_sub(1)),
        };

        if next_col >= cols || next_row >= rows {
            return false;
        }

        if (next_row, next_col) == obstacle_pos || grid[next_row][next_col] == '#' {
            dir.turn();
        } else {
            pos = (next_row, next_col);
        }

        steps += 1;
    }

    false
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_lines_to_vec_vec_char(reader);
        let mut visited = HashSet::new();
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dir = Direction::default();

        // find starting position
        let mut guard_y = (0..rows)
            .position(|r| grid[r].contains(&'^'))
            .expect("Definetly contains '^'");
        let mut guard_x = (0..cols)
            .position(|c| grid[guard_y][c] == '^')
            .expect("Definetly contains '^'");

        // traverse
        loop {
            visited.insert((guard_x, guard_y));
            if guard_exits_grid(guard_x, guard_y, cols, rows, dir) {
                break;
            }

            move_guard(&grid, &mut guard_x, &mut guard_y, &mut dir);
        }

        let answer = visited.len();
        Ok(answer)
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_lines_to_vec_vec_char(reader);
        let (rows, cols) = (grid.len(), grid[0].len());

        let mut start_pos = (0, 0);
        let mut empty_positions = vec![];
        for (i, row) in grid.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if c == '^' {
                    start_pos = (i, j);
                } else if c == '.' {
                    empty_positions.push((i, j));
                }
            }
        }

        let mut valid_positions = vec![];
        for pos in empty_positions {
            if simulate_with_obstacle(&grid, pos, start_pos, rows, cols) {
                valid_positions.push(pos);
            }
        }

        Ok(valid_positions.len())
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
