use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "13";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

const CONVERSION: isize = 10_000_000_000_000;

type Button = (usize, usize);
type Prize = (usize, usize);
type Machine = (Button, Button, Prize);
fn read_input<R: BufRead>(reader: R) -> Result<Vec<Machine>> {
    let mut output: Vec<Machine> = Vec::new();
    let machines = reader.lines().chunks(4);
    for machine in machines.into_iter() {
        let mut button_a: Button = (0, 0);
        let mut button_b: Button = (0, 0);
        let mut prize: Prize = (0, 0);
        for line in machine {
            let line = line?;
            if line.starts_with("Button") {
                let coords = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|x| x.split('+').nth(1).unwrap().parse::<usize>().unwrap())
                    .collect_vec();
                if line.starts_with("Button A") {
                    button_a = (coords[0], coords[1]);
                } else if line.starts_with("Button B") {
                    button_b = (coords[0], coords[1]);
                }
            } else if line.starts_with("Prize") {
                let coords = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|x| x.split('=').nth(1).unwrap().parse::<usize>().unwrap())
                    .collect_vec();
                prize = (coords[0], coords[1]);
            }
        }
        output.push((button_a, button_b, prize));
    }
    Ok(output)
}

fn solve_machine(machine: Machine, offset: isize) -> isize {
    let (ax, ay, bx, by, px, py) = (
        machine.0 .0 as isize,
        machine.0 .1 as isize,
        machine.1 .0 as isize,
        machine.1 .1 as isize,
        machine.2 .0 as isize + offset,
        machine.2 .1 as isize + offset,
    );
    let det = ax * by - ay * bx;
    if det == 0 {
        0
    } else {
        let mut a = px * by - py * bx;
        let mut b = py * ax - px * ay;
        if a % det != 0 || b % det != 0 {
            0
        } else {
            a /= det;
            b /= det;
            if !(a.is_negative() || b.is_negative()) {
                a * 3 + b
            } else {
                0
            }
        }
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<isize> {
        let machines = read_input(reader)?;
        let answer = machines
            .into_iter()
            .map(|machine| solve_machine(machine, 0))
            .sum::<isize>();
        Ok(answer)
    }

    assert_eq!(480, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<isize> {
        let machines = read_input(reader)?;
        let answer = machines
            .into_iter()
            .map(|machine| solve_machine(machine, CONVERSION))
            .sum::<isize>();
        Ok(answer)
    }

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
