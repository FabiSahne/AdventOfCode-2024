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

#[derive(PartialEq, Default)]
enum Direction {
    #[default]
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn(&mut self) {
        use crate::Direction::*;
        match self {
            Up => *self = Right,
            Right => *self = Down,
            Down => *self = Left,
            Left => *self = Up,
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        use crate::Direction::*;
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
            if guard_y == grid.len() - 1 && dir == Down
                || guard_y == 0 && dir == Up
                || guard_x == grid[0].len() - 1 && dir == Right
                || guard_x == 0 && dir == Left
            {
                break;
            }

            match dir {
                Up => {
                    if grid[guard_y - 1][guard_x] == '#' {
                        dir.turn();
                    } else {
                        guard_y -= 1
                    }
                }
                Right => {
                    if grid[guard_y][guard_x + 1] == '#' {
                        dir.turn();
                    } else {
                        guard_x += 1
                    }
                }
                Down => {
                    if grid[guard_y + 1][guard_x] == '#' {
                        dir.turn();
                    } else {
                        guard_y += 1
                    }
                }
                Left => {
                    if grid[guard_y][guard_x - 1] == '#' {
                        dir.turn();
                    } else {
                        guard_x -= 1
                    }
                }
            }
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
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {result}");
    //endregion

    Ok(())
}
