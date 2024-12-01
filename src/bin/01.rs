use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<i32> {
        let (mut left, mut right) = (BinaryHeap::new(), BinaryHeap::new());

        for line in reader.lines() {
            let line = line?;
            let line = line.split_whitespace().collect::<Vec<&str>>();
            left.push(Reverse(line[0].parse::<i32>()?));
            right.push(Reverse(line[1].parse::<i32>()?));
        }

        let mut answer = 0;
        for _ in 0..left.len() {
            let (Reverse(left), Reverse(right)) = (left.pop().unwrap(), right.pop().unwrap());
            answer += (left - right).abs();
        }

        Ok(answer)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<i32> {
        let (mut left, right) =
            reader
                .lines()
                .fold((vec![], vec![]), |(mut left, mut right), line| {
                    let line = line.unwrap();
                    let line = line.split_whitespace().collect::<Vec<&str>>();
                    left.push(line[0].parse::<i32>().unwrap());
                    right.push(line[1].parse::<i32>().unwrap());
                    (left, right)
                });

        let mut freq_table = HashMap::new();
        for n in right {
            freq_table.entry(n).and_modify(|e| *e += 1).or_insert(1);
        }

        for n in left.iter_mut() {
            *n *= freq_table.get(n).unwrap_or(&0);
        }

        let answer = left.into_iter().sum::<i32>();

        Ok(answer)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
