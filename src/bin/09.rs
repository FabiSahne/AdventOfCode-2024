use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::char;
// use std::fmt::Write;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "09";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "2333133121414131402";

fn calculate_checksum(disk: &[Option<usize>]) -> usize {
    disk.iter().enumerate().fold(0, |acc, (idx, v)| match v {
        Some(val) => acc + (idx * val),
        None => acc,
    })
}

fn disk_from_map<R: BufRead>(reader: R) -> Vec<Option<usize>> {
    let disk_map = reader
        .bytes()
        .map_while(Result::ok)
        .map(|b| char::from(b).to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>();

    let mut disk = vec![];
    for (id, chunk) in disk_map.chunks(2).enumerate() {
        let file = chunk[0];
        disk.append(&mut vec![Some(id); file]);
        if chunk.len() > 1 {
            let free = chunk[1];
            disk.append(&mut vec![None; free]);
        }
    }
    disk
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut disk = disk_from_map(reader);

        let mut left = disk.iter().position(|x| x.is_none()).unwrap();
        let mut right = disk.iter().rposition(|x| x.is_some()).unwrap();

        while left < right {
            disk[left] = disk[right].take();

            while left < disk.len() - 1 && disk[left].is_some() {
                left += 1;
            }
            while right > 0 && disk[right].is_none() {
                right -= 1;
            }
        }

        let answer = calculate_checksum(&disk);
        Ok(answer)
    }

    assert_eq!(1928, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let disk = disk_from_map(reader);

        let right_edge = disk.iter().rposition(|x| x.is_some()).unwrap();
        loop {
            let mut left_edge = right_edge;
            while left_edge > 0 && disk[left_edge] == disk[right_edge] {
                left_edge -= 1;
            }
            left_edge += 1;
            let size = right_edge - left_edge + 1;

            let left_free = disk.iter().position(|x| x.is_none()).unwrap();
            let right_free = left_free;
        }

        Ok(0)
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
