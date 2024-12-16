use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::char;
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

fn read_disk_map<R: BufRead>(reader: R) -> Vec<usize> {
    reader
        .bytes()
        .map_while(Result::ok)
        .map(|b| char::from(b).to_digit(10).unwrap() as usize)
        .collect::<Vec<_>>()
}

fn disk_from_map(map: &[usize]) -> Vec<Option<usize>> {
    map.chunks(2)
        .enumerate()
        .fold(vec![], |mut acc, (id, chunk)| {
            let file = chunk[0];
            acc.append(&mut vec![Some(id); file]);
            if chunk.len() > 1 {
                let free = chunk[1];
                acc.append(&mut vec![None; free]);
            }
            acc
        })
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let map = read_disk_map(reader);
        let mut disk = disk_from_map(&map);

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
        let disk_map = read_disk_map(reader);

        let mut disk = disk_map
            .chunks(2)
            .enumerate()
            .fold(vec![], |mut acc, (id, chunk)| {
                let file = chunk[0];
                acc.push((Some(id), file));
                if chunk.len() > 1 {
                    let free = chunk[1];
                    acc.push((None, free));
                }
                acc
            });

        let right_most_file = disk.iter().rposition(|x| x.0.is_some()).unwrap();
        loop {
            let mut left_most_free = disk.iter().position(|x| x.0.is_none()).unwrap();
            while left_most_free < disk.len() && disk[left_most_free].1 < disk[right_most_file].1 {
                left_most_free += 1;
                while disk[left_most_free].0.is_none() {
                    left_most_free += 1;
                }
            }
            if disk[left_most_free].1 >= disk[right_most_file].1 {
                let diff = disk[left_most_free].1 - disk[right_most_file].1;
                disk[left_most_free] = disk[right_most_file];
                disk.insert(left_most_free + 1, (None, disk[left_most_free].1 - diff));
                disk[right_most_file + 1] = (None, disk[right_most_file + 1].1);
            }
        }
    }

    assert_eq!(2858, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
