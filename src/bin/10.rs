use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use pathfinding::prelude::count_paths;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "10";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

// const DIRECTIONS: [[usize; 2]; 4] = [[0, 1], [1, 0], [0, usize::MAX], [usize::MAX, 0]];

fn read_map<R: BufRead>(reader: R) -> Vec<Vec<usize>> {
    reader
        .lines()
        .map_while(|line| line.ok())
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let topo_map = read_map(reader);
        let mut answer = 0;
        for y in 0..topo_map.len() {
            for x in 0..topo_map[0].len() {
                if topo_map[y][x] == 0 {
                    answer += count_paths(
                        (x, y),
                        |&(x, y)| {
                            let target = topo_map[y][x] + 1;
                            let mut succ = vec![
                                (x + 1, y),
                                (x, y + 1),
                                (x.wrapping_sub(1), y),
                                (x, y.wrapping_sub(1)),
                            ];
                            succ.retain(|&(x, y)| {
                                x < topo_map[0].len()
                                    && y < topo_map.len()
                                    && topo_map[y][x] == target
                            });
                            succ
                        },
                        |&(x, y)| topo_map[y][x] == 9,
                    );
                }
            }
        }
        Ok(answer)
    }

    assert_eq!(36, part1(BufReader::new(TEST.as_bytes()))?);

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
