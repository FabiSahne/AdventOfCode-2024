use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use itertools::Itertools;
use pathfinding::prelude::{astar, dijkstra};

const DAY: &str = "21";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
029A
980A
179A
456A
379A
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let successors = |n: char| {
            match n {
                'A' => vec![('3', 1), ('0', 1)],
                '0' => vec![('2', 1), ('A', 1)],
                '1' => vec![('2', 1), ('4', 1)],
                '2' => vec![],
                _ => unreachable!()
            }
        };
        let re = Regex::new(r"(?<d>\d+)A")?;
        let mut answer = 0;
        for line_result in reader.lines() {
            let line = line_result?;
            let numeric_part = re
                .captures(&line)
                .map(|cap| cap["d"].parse::<usize>().unwrap())
                .unwrap();

            let mut current = 'A';
            let mut path = vec![];

            for (start, goal) in line.chars().tuple_windows() {
                path.append(&mut dijkstra(&start, |&n| successors(n), |&n| n == goal).expect("No Path").0)
            }

            answer += numeric_part;
        }

        Ok(answer)
    }

    assert_eq!(126384, part1(BufReader::new(TEST.as_bytes()))?);

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
