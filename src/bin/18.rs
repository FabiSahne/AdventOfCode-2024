use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use pathfinding::num_traits::WrappingSub;
use pathfinding::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "18";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

macro_rules! succsessors {
    ($mem:ident, $size:ident) => {
        |(x, y)| {
            vec![
                ((x + 1, *y), 1),
                ((*x, y + 1), 1),
                ((x.wrapping_sub(&1), *y), 1),
                ((*x, y.wrapping_sub(&1)), 1),
            ]
            .into_iter()
            .filter(|((x, y), _)| *x < $size && *y < $size && $mem[*y][*x] != '#')
            .collect_vec()
        }
    };
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, size: usize, ns: usize) -> Result<usize> {
        let mut mem = vec![vec!['.'; size]; size];

        let mut lines = reader.lines();

        for _s in 0..ns {
            let (x, y) = lines
                .next()
                .unwrap()?
                .split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap();
            mem[y][x] = '#'
        }

        //print_map(&mem);

        let (_, pathlen) = dijkstra(&(0usize, 0usize), succsessors!(mem, size), |n| {
            n == &(size - 1, size - 1)
        })
        .expect("No Path found");

        Ok(pathlen)
    }

    assert_eq!(22, part1(BufReader::new(TEST.as_bytes()), 7, 12)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, 71, 1024)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, size: usize) -> Result<(usize, usize)> {
        let mut mem = vec![vec!['.'; size]; size];

        let mut path = (0..size).map(|i| (0, i)).collect_vec();
        path.append(&mut (0..size).map(|i| (i, size - 1)).collect_vec());

        for line in reader.lines() {
            let (x, y) = line?
                .split(',')
                .map(|d| d.parse::<usize>().unwrap())
                .next_tuple()
                .unwrap();
            mem[y][x] = '#';

            // print_map(&mem);

            if path.contains(&(x, y)) {
                let astar_result = astar(
                    &(0usize, 0usize),
                    succsessors!(mem, size),
                    |(x, y)| (size - 1 - x) + (size - 1 - y),
                    |&n| n == (size - 1, size - 1),
                );
                match astar_result {
                    Some((v, _)) => path = v,
                    None => return Ok((x, y)),
                }
                // println!("{path:?}");
            }
        }
        Err(Error::msg("No Broken Path"))
    }

    assert_eq!((6, 1), part2(BufReader::new(TEST.as_bytes()), 7)?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 71)?);
    println!("Result = {},{}", result.0, result.1);
    //endregion

    Ok(())
}
