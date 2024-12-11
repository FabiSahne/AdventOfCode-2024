use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
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

const DIRECTIONS: [[usize; 2]; 4] = [[0, 1], [1, 0], [0, usize::MAX], [usize::MAX, 0]];

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

// fn is_trailhead(map: &[Vec<usize>], i: usize, j: usize, height: usize) -> bool {
//     if i > map.len() - 1 || j > map[0].len() - 1 || map[i][j] != height {
//         false
//     } else if height == 9 {
//         true
//     } else {
//         let mut part_of_trail = false;
//         for dir in DIRECTIONS {
//             if is_trailhead(
//                 map,
//                 i.wrapping_add(dir[0]),
//                 j.wrapping_add(dir[1]),
//                 height + 1,
//             ) {
//                 part_of_trail = true;
//                 break;
//             }
//         }
//         part_of_trail
//     }
//}

fn trailhead_score(map: &[Vec<usize>], i: usize, j: usize, height: usize) -> usize {
    if i > map.len() - 1 || j > map[0].len() - 1 || map[i][j] != height {
        0
    } else if height == 9 {
        1
    } else {
        let mut score = 0;
        for dir in DIRECTIONS {
            score += trailhead_score(
                map,
                i.wrapping_add(dir[0]),
                j.wrapping_add(dir[1]),
                height + 1,
            );
        }
        score
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let topo_map = read_map(reader);
        let mut answer = 0;
        for (i, row) in topo_map.iter().enumerate() {
            for (j, height) in row.iter().enumerate() {
                if *height == 0 {
                    answer += trailhead_score(&topo_map, i, j, 0);
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
