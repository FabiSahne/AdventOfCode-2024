use crate::Direction::*;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
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

#[derive(PartialEq, Default, Copy, Clone)]
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
                move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_y -= 1
            }
        }
        Right => {
            if grid[*guard_y][*guard_x + 1] == '#' {
                dir.turn();
                move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_x += 1
            }
        }
        Down => {
            if grid[*guard_y + 1][*guard_x] == '#' {
                dir.turn();
                move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_y += 1
            }
        }
        Left => {
            if grid[*guard_y][*guard_x - 1] == '#' {
                dir.turn();
                move_guard(grid, guard_x, guard_y, dir);
            } else {
                *guard_x -= 1
            }
        }
    };
}

fn check_loop(
    mut grid: Vec<Vec<char>>,
    mut visited: HashMap<(usize, usize), Direction>,
    mut guard_x: usize,
    mut guard_y: usize,
    mut dir: Direction,
) -> bool {
    match dir {
        Up => grid[guard_x][guard_y - 1] = '#',
        Right => grid[guard_x + 1][guard_y] = '#',
        Down => grid[guard_x][guard_y + 1] = '#',
        Left => grid[guard_x - 1][guard_y] = '#',
    };

    loop {
        visited.insert((guard_x, guard_y), dir);

        if guard_exits_grid(guard_x, guard_y, grid[0].len(), grid.len(), dir) {
            return false;
        }

        move_guard(&grid, &mut guard_x, &mut guard_y, &mut dir);

        if visited.get(&(guard_x, guard_y)) == Some(&dir) {
            return true;
        }
    }
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
        let mut visited = HashMap::new();
        let mut dir = Direction::default();

        let mut guard_y = (0..rows)
            .position(|r| grid[r].contains(&'^'))
            .expect("Definetly contains '^'");
        let mut guard_x = (0..cols)
            .position(|c| grid[guard_y][c] == '^')
            .expect("Definetly contains '^'");

        let mut loops = 0;

        loop {
            visited.insert((guard_x, guard_y), dir);
            if guard_exits_grid(guard_x, guard_y, cols, rows, dir) {
                break;
            }

            if check_loop(grid.clone(), visited.clone(), guard_y, guard_x, dir) {
                loops += 1;
            }

            move_guard(&grid, &mut guard_x, &mut guard_y, &mut dir);
        }

        Ok(loops)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
