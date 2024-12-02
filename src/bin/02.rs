#![allow(clippy::wildcard_imports)]
#![allow(clippy::items_after_statements)]
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    println!("=== Part 1 ===");

    //region Part 1
    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut reports = vec![];
        for rline in reader.lines() {
            let line = rline?;
            reports.push(
                line.split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap_or(0))
                    .collect::<Vec<i32>>(),
            );
        }

        let answer = reports.into_iter().filter(|r| is_safe(r)).count();

        Ok(answer)
    }
    fn is_safe(report: &[i32]) -> bool {
        assert!(report.len() >= 2);
        if report[1] > report[0] && report[1] <= report[0] + 3 {
            return is_ascending(report);
        }
        if report[0] > report[1] && report[0] <= report[1] + 3 {
            return is_descending(report);
        }
        false
    }
    fn is_ascending(report: &[i32]) -> bool {
        report.windows(2).all(|w| w[1] > w[0] && w[1] <= w[0] + 3)
    }
    fn is_descending(report: &[i32]) -> bool {
        report.windows(2).all(|w| w[0] > w[1] && w[0] <= w[1] + 3)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // #[allow(clippy::items_after_statements)]
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
