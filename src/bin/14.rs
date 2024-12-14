use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "14";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

const SECONDS: isize = 100;

struct Robot {
    x: isize,
    y: isize,
    vx: isize,
    vy: isize,
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, width: isize, height: isize) -> Result<usize> {
        let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)")?;
        let mut robots = vec![];
        for line in reader.lines() {
            let line = line?;
            let caps = re.captures(&line).ok_or(anyhow!("Malformed input"))?;
            let x = caps["px"].parse::<isize>()?;
            let y = caps["py"].parse::<isize>()?;
            let vx = caps["vx"].parse::<isize>()?;
            let vy = caps["vy"].parse::<isize>()?;
            robots.push(Robot { x, y, vx, vy });
        }

        for robot in robots.iter_mut() {
            robot.x = ((robot.x + robot.vx * SECONDS) + width * SECONDS) % (width);
            robot.y = ((robot.y + robot.vy * SECONDS) + height * SECONDS) % (height);
            println!("p={},{}", robot.x, robot.y);
        }

        let mut top_left = 0;
        let mut bottom_left = 0;
        let mut top_right = 0;
        let mut bottom_right = 0;

        for robot in robots {
            match (robot.x, robot.y) {
                (x, y) if x < width / 2 && y < height / 2 => top_left += 1,
                (x, y) if x < width / 2 && y > height / 2 => bottom_left += 1,
                (x, y) if x > width / 2 && y < height / 2 => top_right += 1,
                (x, y) if x > width / 2 && y > height / 2 => bottom_right += 1,
                _ => {}
            }
        }
        println!("{top_left}, {top_right}, {bottom_left}, {bottom_right}");
        let answer = top_left * bottom_left * top_right * bottom_right;
        Ok(answer)
    }

    assert_eq!(12, part1(BufReader::new(TEST.as_bytes()), 11, 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 101, 103)?);
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
