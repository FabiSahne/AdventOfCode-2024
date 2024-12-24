use crate::Operation::*;
use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use regex::Regex;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::str::FromStr;

const DAY: &str = "24";

const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

enum Operation {
    Or,
    And,
    Xor,
}

impl FromStr for Operation {
    type Err = io::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "or" => Result::Ok(Or),
            "and" => Result::Ok(And),
            "xor" => Result::Ok(Xor),
            _ => Err(io::Error::from(ErrorKind::InvalidData)),
        }
    }
}

impl Operation {
    fn calc(&self, rhs: usize, lhs: usize) -> usize {
        match self {
            Or => rhs | lhs,
            And => rhs & lhs,
            Xor => rhs ^ lhs,
        }
    }
}
fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut wires = HashMap::new();
        let mut queue = VecDeque::new();
        let mut done_reading_wirestates = false;

        let re_op = Regex::new(r"(?<lhs>.+) (?<op>.+) (?<rhs>.+) -> (?<res>.+)")?;
        let re_wire = Regex::new(r"(?<wire>.+): (?<state>\d)")?;

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() {
                done_reading_wirestates = true;
            } else if done_reading_wirestates {
                let (_, [lhs, op, rhs, res]) = re_op.captures(&line).unwrap().extract();
                queue.push_back((
                    (lhs.to_owned(), rhs.to_owned()),
                    res.to_owned(),
                    Operation::from_str(op)?,
                ))
            } else {
                let (_, [wire, state]) = re_wire.captures(&line).unwrap().extract();
                wires.insert(wire.to_owned(), usize::from_str(state)?);
            }
        }

        while !queue.is_empty() {
            let ((lhs, rhs), res, op) = queue.pop_front().unwrap();
            if wires.contains_key(&lhs) && wires.contains_key(&rhs) {
                wires.insert(res, op.calc(wires[&lhs], wires[&rhs]));
            } else {
                queue.push_back(((lhs, rhs), res, op));
            }
        }

        let re_zdd = Regex::new(r"z\d\d")?;

        let answer = wires
            .into_iter()
            .filter(|(k, _)| re_zdd.is_match(k))
            .sorted_unstable_by(|(ka, _), (kb, _)| kb.cmp(ka))
            .fold(0, |acc, (_, v)| (acc << 1) | v);
        Ok(answer)
    }

    assert_eq!(2024, part1(BufReader::new(TEST.as_bytes()))?);

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
