use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "12";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

const DIRECTIONS: [[usize; 2]; 4] = [[0, 1], [1, 0], [0, usize::MAX], [usize::MAX, 0]];

fn get_plot_stats(
    garden: &[Vec<char>],
    i: usize,
    j: usize,
    ch: char,
    visited: &mut HashSet<(usize, usize)>,
) -> Option<(usize, usize)> {
    if visited.contains(&(i, j)) && ch == garden[i][j] {
        None
    } else if i > garden.len() - 1 || j > garden[0].len() - 1 || ch != garden[i][j] {
        Some((0, 1))
    } else {
        visited.insert((i, j));
        let mut area = 1;
        let mut perimeter = 0;
        for dir in DIRECTIONS {
            if let Some((a, p)) = get_plot_stats(
                garden,
                i.wrapping_add(dir[0]),
                j.wrapping_add(dir[1]),
                ch,
                visited,
            ) {
                area += a;
                perimeter += p;
            }
        }
        Some((area, perimeter))
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let garden = read_lines_to_vec_vec_char(reader);
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut plots = vec![];

        for (i, row) in garden.iter().enumerate() {
            for (j, &ch) in row.iter().enumerate() {
                if !visited.contains(&(i, j)) {
                    plots.push(get_plot_stats(&garden, i, j, ch, &mut visited));
                }
            }
        }

        let answer = plots
            .into_iter()
            .map(|p| {
                if let Some((area, peri)) = p {
                    area * peri
                } else {
                    0
                }
            })
            .sum();
        Ok(answer)
    }

    assert_eq!(1930, part1(BufReader::new(TEST.as_bytes()))?);

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
