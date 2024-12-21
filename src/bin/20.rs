use adv_code_2024::Direction::{East, North, South, West};
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use pathfinding::prelude::{count_paths, dijkstra};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = read_lines_to_vec_vec_char(reader);
        let (start, goal) = get_start_and_goal_from_grid(&grid);
        let mut allowed_cheat = 2;
        let (_, normal_path) = dijkstra(
            &start,
            |&n| {
                vec![(n + North, 1), (n + West, 1), (n + South, 1), (n + East, 1)]
                    .into_iter()
                    .filter(|(n, _)| {
                        n.x < grid[0].len() && n.y < grid.len() && grid[n.y][n.x] != '#'
                    })
                    .collect::<Vec<(_, usize)>>()
            },
            |&n| n == goal,
        )
        .expect("No Path found!");

        let answer = count_paths(
            (start, 0usize),
            |&(n, c)| {
                if allowed_cheat > 0 {
                    allowed_cheat -= 1;
                    [n + North, n + East, n + South, n + West]
                        .into_iter()
                        .filter(|&n| n.x < grid[0].len() && n.y < grid.len())
                        .map(|n| (n, c + 1))
                        .collect::<Vec<_>>()
                } else {
                    [n + North, n + East, n + South, n + West]
                        .into_iter()
                        .filter(|&n| {
                            n.x < grid[0].len() && n.y < grid.len() && grid[n.y][n.x] != '#'
                        })
                        .map(|n| (n, c + 1))
                        .collect()
                }
            },
            |&(n, c)| n == goal && c <= normal_path - 100,
        );
        Ok(answer)
    }

    // TODO: Set the expected answer for the test input
    //assert_eq!(0, part1(BufReader::new(TEST.as_bytes()))?);

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
