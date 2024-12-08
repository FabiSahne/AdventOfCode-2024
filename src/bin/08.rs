use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "08";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

fn get_antinodes(
    node1: (usize, usize),
    node2: (usize, usize),
    max_x: usize,
    max_y: usize,
) -> Vec<(usize, usize)> {
    fn is_valid_antinode(node: (i32, i32), max_x: usize, max_y: usize) -> bool {
        node.0 >= 0 && node.1 >= 0 && (node.0 as usize) < max_x && (node.1 as usize) < max_y
    }
    let (node1_x, node1_y) = (node1.0 as i32, node1.1 as i32);
    let (node2_x, node2_y) = (node2.0 as i32, node2.1 as i32);

    let mut nodes = vec![];

    let x_diff = node2_x - node1_x;
    let y_diff = node2_y - node1_y;

    let antinode1 = (node1_x - x_diff, node1_y - y_diff);
    let antinode2 = (node2_x + x_diff, node2_y + y_diff);

    if is_valid_antinode(antinode1, max_x, max_y) {
        nodes.push((antinode1.0 as usize, antinode1.1 as usize));
    }
    if is_valid_antinode(antinode2, max_x, max_y) {
        nodes.push((antinode2.0 as usize, antinode2.1 as usize));
    }
    nodes
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut data = read_lines_to_vec_vec_char(reader);
        let mut node_map = HashMap::new(); // (x, y) coords for every type of node
        for (y, line) in data.iter().enumerate() {
            for (x, &ch) in line.iter().enumerate() {
                if ch.is_alphanumeric() {
                    node_map.entry(ch).or_insert_with(Vec::new).push((x, y));
                }
            }
        }

        let (x, y) = (data[0].len(), data.len());
        let mut answer = 0;

        for (_, coords) in node_map {
            for pair in coords.iter().combinations(2) {
                let node1 = *pair[0];
                let node2 = *pair[1];

                for antinode in get_antinodes(node1, node2, x, y) {
                    if data[antinode.1][antinode.0] != '#' {
                        answer += 1;
                        data[antinode.1][antinode.0] = '#';
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(14, part1(BufReader::new(TEST.as_bytes()))?);

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
