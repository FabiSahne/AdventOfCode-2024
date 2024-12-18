use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_1: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

const TEST_2: &str = "\
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

const THREADS: usize = 16;

macro_rules! combo {
    ($operand:expr, $reg_a:expr, $reg_b:expr, $reg_c:expr) => {
        match $operand {
            0..=3 => $operand,
            4 => $reg_a,
            5 => $reg_b,
            6 => $reg_c,
            _ => 7,
        }
    };
}

fn adv(reg_a: usize, combo: usize) -> usize {
    let denominator = 2usize.pow(combo as u32);
    reg_a / denominator
}

fn bxl(reg_b: usize, literal: usize) -> usize {
    reg_b ^ literal
}

fn bst(combo: usize) -> usize {
    combo % 8
}

fn jnz(reg_a: usize, literal: usize) -> Option<usize> {
    if reg_a == 0 {
        None
    } else {
        Some(literal)
    }
}

fn bxc(reg_b: usize, reg_c: usize) -> usize {
    reg_b ^ reg_c
}

fn out(combo: usize, output: &mut String) {
    if output.is_empty() {
        output.push_str(&format!("{}", bst(combo)));
    } else {
        output.push_str(&format!(",{}", bst(combo)));
    }
}

fn bdv(reg_a: usize, combo: usize) -> usize {
    adv(reg_a, combo)
}

fn cdv(reg_a: usize, combo: usize) -> usize {
    adv(reg_a, combo)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<String> {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        let reg_reg = Regex::new(r"Register .: (?<v>\d+)")?;
        let (mut reg_a, mut reg_b, mut reg_c) = reg_reg
            .captures_iter(&text)
            .map(|caps| caps["v"].parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();
        let pro_reg = Regex::new(r"Program: (?<p>.*)")?;
        let program = pro_reg
            .captures(&text)
            .map(|caps| {
                caps["p"]
                    .chars()
                    .filter(|c| c.is_ascii_digit())
                    .map(|c| (c as u8 - b'0') as usize)
                    .collect_vec()
            })
            .unwrap();

        let mut output = String::new();
        let mut pc = 0;

        while pc < program.len() {
            // println!(
            //     "Register A: {}\nRegister B: {}\nRegister C: {}\nInstruction: ({}, {})\n",
            //     reg_a,
            //     reg_b,
            //     reg_c,
            //     program[pc],
            //     program[pc + 1]
            // );
            match program[pc] {
                0 => reg_a = adv(reg_a, combo!(program[pc + 1], reg_a, reg_b, reg_c)),
                1 => reg_b = bxl(reg_b, program[pc + 1]),
                2 => reg_b = bst(combo!(program[pc + 1], reg_a, reg_b, reg_c)),
                3 => {
                    pc = jnz(reg_a, program[pc + 1]).unwrap_or(pc + 2);
                    continue;
                }
                4 => reg_b = bxc(reg_b, reg_c),
                5 => out(combo!(program[pc + 1], reg_a, reg_b, reg_c), &mut output),
                6 => reg_b = bdv(reg_a, combo!(program[pc + 1], reg_a, reg_b, reg_c)),
                _ => reg_c = cdv(reg_a, combo!(program[pc + 1], reg_a, reg_b, reg_c)),
            }

            pc += 2;
        }

        Ok(output)
    }

    assert_eq!(
        "4,6,3,5,6,3,5,2,1,0".to_string(),
        part1(BufReader::new(TEST_1.as_bytes()))?
    );

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut text = String::new();
        reader.read_to_string(&mut text)?;
        let reg_reg = Regex::new(r"Register .: (?<v>\d+)")?;
        let (_, reg_b, reg_c) = reg_reg
            .captures_iter(&text)
            .map(|caps| caps["v"].parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();
        let pro_reg = Regex::new(r"Program: (?<p>.*)")?;
        let program: Arc<String> = Arc::new(pro_reg.captures(&text).unwrap()["p"].to_string());

        let mut handles = vec![];

        for thrd in 0..THREADS {
            let arc = program.clone();
            handles.push(thread::spawn(move || {
                for a in (thrd..).step_by(THREADS) {
                    let new_program = format!(
                        "Register A: {}\nRegister B: {}\nRegister C: {}\nProgram: {}\n",
                        a, reg_b, reg_c, arc
                    );
                    if arc[..] == part1(BufReader::new(new_program.as_bytes()))? {
                        return Ok(a);
                    }
                }
                unreachable!()
            }));
        }

        while !handles.iter().any(|h| h.is_finished()) {
            thread::sleep(Duration::from_millis(100));
        }
        for h in handles {
            if h.is_finished() {
                return h.join().unwrap();
            }
        }

        unreachable!()
    }

    assert_eq!(117440, part2(BufReader::new(TEST_2.as_bytes()))?);
    println!("Test finished");

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
