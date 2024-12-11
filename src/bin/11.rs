use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let mut stones = buf
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .map_while(Result::ok)
            .collect::<Vec<usize>>();

        for _ in 0..25 {
            //println!("{stones:?}");
            let mut new_stones = vec![];
            for &stone in &stones {
                if stone == 0 {
                    new_stones.push(1);
                } else if (stone.ilog10() + 1) % 2 == 0 {
                    new_stones.push(stone / 10usize.pow((stone.ilog10() + 1) / 2));
                    new_stones.push(stone % 10usize.pow((stone.ilog10() + 1) / 2));
                } else {
                    new_stones.push(stone * 2024);
                }
            }
            stones = new_stones;
        }

        let answer = stones.len();
        Ok(answer)
    }

    assert_eq!(55312, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut buf = String::new();
        reader.read_to_string(&mut buf)?;
        let stones = buf
            .split_whitespace()
            .map(|s| s.parse::<usize>())
            .map_while(Result::ok)
            .collect::<Vec<usize>>();

        let mut freq_count = HashMap::new();
        for stone in stones {
            freq_count.entry(stone).and_modify(|x| *x += 1).or_insert(1);
        }

        for _ in 0..75 {
            for (stone, count) in freq_count.clone() {
                freq_count.entry(stone).and_modify(|x| *x -= count);
                if stone == 0 {
                    freq_count
                        .entry(1)
                        .and_modify(|x| *x += count)
                        .or_insert(count);
                } else if (stone.ilog10() + 1) % 2 == 0 {
                    // new_stones.push(stone / 10usize.pow((stone.ilog10() + 1) / 2));
                    // new_stones.push(stone % 10usize.pow((stone.ilog10() + 1) / 2));
                    freq_count
                        .entry(stone / 10usize.pow((stone.ilog10() + 1) / 2))
                        .and_modify(|x| *x += count)
                        .or_insert(count);
                    freq_count
                        .entry(stone % 10usize.pow((stone.ilog10() + 1) / 2))
                        .and_modify(|x| *x += count)
                        .or_insert(count);
                } else {
                    freq_count
                        .entry(stone * 2024)
                        .and_modify(|x| *x += count)
                        .or_insert(count);
                }
            }
        }

        Ok(freq_count.values().sum())
    }

    //assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
