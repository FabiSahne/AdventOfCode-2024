use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

const DIRECTIONS: [[usize; 2]; 8] = [
    [1, 0],                   // right
    [1, 1],                   // down, right
    [0, 1],                   // down
    [usize::MAX, 1],          // down, left
    [usize::MAX, 0],          // left
    [usize::MAX, usize::MAX], // up, left
    [0, usize::MAX],          // up
    [1, usize::MAX],          // up right
];

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];

fn is_xmas(text: &[Vec<char>], row: usize, col: usize, dir: [usize; 2], xmas: usize) -> bool {
    if xmas == 4 {
        true
    } else if row == 0 && dir[0] == usize::MAX
        || col == 0 && dir[1] == usize::MAX
        || row.wrapping_add(dir[0]) == text.len()
        || col.wrapping_add(dir[1]) == text[0].len()
    {
        false
    } else {
        let next_char = XMAS[xmas];
        if text[row.wrapping_add(dir[0])][col.wrapping_add(dir[1])] == next_char {
            is_xmas(
                text,
                row.wrapping_add(dir[0]),
                col.wrapping_add(dir[1]),
                dir,
                xmas + 1,
            )
        } else {
            false
        }
    }
}

fn is_x_mas(text: &[Vec<char>], row: usize, col: usize) -> bool {
    if row == 0 || col == 0 || row == text.len() - 1 || col == text[0].len() - 1 {
        false
    } else {
        let up_left_down_right = [text[row + 1][col + 1], text[row - 1][col - 1]];
        let up_right_down_left = [text[row + 1][col - 1], text[row - 1][col + 1]];
        (up_left_down_right == ['M', 'S'] || up_left_down_right == ['S', 'M'])
            && (up_right_down_left == ['M', 'S'] || up_right_down_left == ['S', 'M'])
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let text = read_lines_to_vec_vec_char(reader);

        let mut answer = 0;
        for (row_num, row) in text.iter().enumerate() {
            for (col_num, c) in row.iter().enumerate() {
                if c == &'X' {
                    for dir in DIRECTIONS {
                        if is_xmas(&text, row_num, col_num, dir, 1) {
                            answer += 1;
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let text = read_lines_to_vec_vec_char(reader);

        let (n, m) = (text.len(), text[0].len());

        let mut answer = 0;
        for row in 0..n {
            for col in 0..m {
                if text[row][col] == 'A' && is_x_mas(&text, row, col) {
                    answer += 1;
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(9, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
