use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn read_input_to_equation<R: BufRead, T: FromStr + Default>(reader: R) -> Vec<(T, Vec<T>)> {
    reader
        .lines()
        .map(|l| {
            let line = l.expect("line should contain data");
            let eq = line.split(':').collect::<Vec<&str>>();
            let values = eq[1]
                .split_whitespace()
                .map(|s| s.parse::<T>().unwrap_or_default())
                .collect::<Vec<T>>();
            (eq[0].parse::<T>().unwrap_or_default(), values)
        })
        .collect::<Vec<_>>()
}

#[inline]
fn concat(p: usize, q: usize) -> usize {
    p * 10usize.pow(q.ilog10() + 1) + q
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let lines = read_input_to_equation::<R, usize>(reader);
        let mut answer = 0;

        'next: for (goal, values) in lines {
            let n = values.len();
            let mut grid = vec![vec![vec![]; n]; n];
            grid[0][0].push(values[0]);
            'current: for i in 0..n {
                for j in 0..n {
                    if i + j == n {
                        continue 'current;
                    }
                    if i > 0 {
                        for num in grid[i - 1][j].clone() {
                            grid[i][j].push(num * values[i + j]);
                        }
                    }
                    if j > 0 {
                        for num in grid[i][j - 1].clone() {
                            grid[i][j].push(num + values[i + j]);
                        }
                    }
                    if i + j == n - 1 && grid[i][j].contains(&goal) {
                        answer += goal;
                        continue 'next;
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let lines = read_input_to_equation::<R, usize>(reader);

        let mut answer = 0;

        'next: for (goal, values) in lines {
            let n = values.len();
            let mut grid = vec![vec![vec![vec![]; n]; n]; n];
            grid[0][0][0].push(values[0]);
            for i in 0..n {
                'current: for j in 0..n {
                    for k in 0..n {
                        if i + j + k >= n {
                            continue 'current;
                        }
                        if i > 0 {
                            for num in grid[i - 1][j][k].clone() {
                                grid[i][j][k].push(num * values[i + j + k]);
                            }
                        }
                        if j > 0 {
                            for num in grid[i][j - 1][k].clone() {
                                grid[i][j][k].push(num + values[i + j + k]);
                            }
                        }
                        if k > 0 {
                            for num in grid[i][j][k - 1].clone() {
                                grid[i][j][k].push(concat(num, values[i + j + k]));
                            }
                        }
                        if i + j + k == n - 1 && grid[i][j][k].contains(&goal) {
                            answer += goal;
                            continue 'next;
                        }
                    }
                }
            }
        }

        Ok(answer)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn concat_test() {
        assert_eq!(12345, concat(12, 345));
        assert_eq!(1234, concat(12, 34));
        assert_eq!(123456, concat(123, 456));
        assert_eq!(12, concat(1, 2));
    }
}
