use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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

const DIRECTIONS: [[usize; 2]; 4] = [[0, usize::MAX], [1, 0], [0, 1], [usize::MAX, 0]];

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_lines_to_vec_vec_char(reader);
        let mut visited = HashSet::new();
        let (rows, cols) = (grid.len(), grid[0].len());
        let mut dir = 0;

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
            if guard_y == grid.len() - 1 && dir == 2        // reached bottom
                || guard_y == 0 && dir == 0                 // reached top
                || guard_x == grid[0].len() - 1 && dir == 1 // reached right
                || guard_x == 0 && dir == 3
            // reached left
            {
                break;
            }

            match dir {
                0 => {
                    if grid[guard_y - 1][guard_x] == '#' {
                        dir = (dir + 1) % 4;
                    } else {
                        guard_y -= 1
                    }
                }
                1 => {
                    if grid[guard_y][guard_x + 1] == '#' {
                        dir = (dir + 1) % 4;
                    } else {
                        guard_x += 1
                    }
                }
                2 => {
                    if grid[guard_y + 1][guard_x] == '#' {
                        dir = (dir + 1) % 4;
                    } else {
                        guard_y += 1
                    }
                }
                3 => {
                    if grid[guard_y][guard_x - 1] == '#' {
                        dir = (dir + 1) % 4;
                    } else {
                        guard_x -= 1
                    }
                }
                _ => unreachable!(),
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
