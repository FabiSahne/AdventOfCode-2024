use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn calc(text: &str) -> Result<usize> {
    let re = Regex::new(r"mul\((?<m1>[0-9]+),(?<m2>[0-9]+)\)")?;
    Ok(re
        .captures_iter(text)
        .map(|caps| {
            let m1 = caps["m1"].parse::<usize>().expect("m1 has to be a number");
            let m2 = caps["m2"].parse::<usize>().expect("m2 has to be a number");
            m1 * m2
        })
        .sum())
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;

        calc(text.as_str())
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;

        while let Some(dont_pos) = text.find("don't()") {
            if let Some(do_pos) = text.find("do()") {
                if do_pos < dont_pos {
                    text.replace_range(do_pos..(do_pos + 4), "");
                } else {
                    text.replace_range(dont_pos..(do_pos + 4), "");
                }
            } else {
                text.replace_range(dont_pos.., "");
            }
        }

        calc(text.as_str())
    }

    assert_eq!(48, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
