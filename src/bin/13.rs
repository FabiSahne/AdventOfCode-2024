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

#[derive(Default)]
struct IVec2 {
    x: isize,
    y: isize,
}
struct Machine {
    a: IVec2,
    b: IVec2,
    p: IVec2,
}
fn read_input<R: BufRead>(reader: R) -> Result<Vec<Machine>> {
    let mut output: Vec<Machine> = Vec::new();
    let machines = reader.lines().chunks(4);
    for machine in machines.into_iter() {
        let mut button_a = IVec2::default();
        let mut button_b = IVec2::default();
        let mut prize = IVec2::default();
        for line in machine {
            let line = line?;
            if line.starts_with("Button") {
                let coords = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|x| x.split('+').nth(1).unwrap().parse::<isize>().unwrap())
                    .collect_vec();
                if line.starts_with("Button A") {
                    button_a = IVec2 {
                        x: coords[0],
                        y: coords[1],
                    };
                } else if line.starts_with("Button B") {
                    button_b = IVec2 {
                        x: coords[0],
                        y: coords[1],
                    };
                }
            } else if line.starts_with("Prize") {
                let coords = line
                    .split(':')
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|x| x.split('=').nth(1).unwrap().parse::<isize>().unwrap())
                    .collect_vec();
                prize = IVec2 {
                    x: coords[0],
                    y: coords[1],
                };
            }
        }
        output.push(Machine {
            a: button_a,
            b: button_b,
            p: prize,
        });
    }
    Ok(output)
}

fn solve_machine(machine: Machine, offset: isize) -> isize {
    let prize = IVec2 {
        x: machine.p.x + offset,
        y: machine.p.y + offset,
    };
    let det = machine.a.x * machine.b.y - machine.a.y * machine.b.x;
    if det == 0 {
        return 0;
    }
    let mut a = prize.x * machine.b.y - prize.y * machine.b.x;
    let mut b = prize.y * machine.a.x - prize.x * machine.a.y;
    if a % det != 0 || b % det != 0 {
        return 0;
    }
    a /= det;
    b /= det;
    if !(a.is_negative() || b.is_negative()) {
        a * 3 + b
    } else {
        0
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
