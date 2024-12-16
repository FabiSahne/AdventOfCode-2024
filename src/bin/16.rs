use adv_code_2024::*;
use anyhow::Result;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use pathfinding::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "16";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_SMALL: &str = "\
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

const TEST_LARGE: &str = "\
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

fn successors(
    (pos, dir): &(Vec2<usize>, Direction),
    grid: &[Vec<char>],
) -> Vec<((Vec2<usize>, Direction), usize)> {
    let mut succ = (-1..=1)
        .map(|i| {
            if i == 0 {
                ((*pos + *dir, *dir), 1)
            } else {
                ((*pos, *dir + i), 1000)
            }
        })
        .collect_vec();

    succ.retain(|((p, _), _)| grid[p.y][p.x] != '#');

    succ
}

fn part1<R: BufRead>(reader: R) -> Result<usize> {
    let grid = read_lines_to_vec_vec_char(reader);
    let (start, goal) = ['S', 'E']
        .iter()
        .map(|target| {
            Vec2::from(
                grid.iter()
                    .enumerate()
                    .filter(|(_, row)| row.contains(target))
                    .map(|(y, row)| {
                        let x = row.iter().position(|c| c == target).unwrap();
                        (x, y)
                    })
                    .next()
                    .unwrap(),
            )
        })
        .next_tuple()
        .unwrap();
    let start_direction = Direction::East;

    let result = dijkstra(
        &(start, start_direction),
        |p| successors(p, &grid),
        |p| p.0 == goal,
    )
    .expect("No Path found");

    // let result = astar(
    //     &(start, start_direction),
    //     |p| successors(p, &grid),
    //     |(p, _)| p.x.abs_diff(goal.x) + p.y.abs_diff(goal.y),
    //     |p| p.0 == goal,
    // )
    // .expect("No Path found");

    Ok(result.1)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    assert_eq!(7036, part1(BufReader::new(TEST_SMALL.as_bytes()))?);
    assert_eq!(11048, part1(BufReader::new(TEST_LARGE.as_bytes()))?);

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn straight_line() {
        const TEST: &str = "\
#######
#S...E#
#######
";
        assert_eq!(4, part1(BufReader::new(TEST.as_bytes())).unwrap());
    }

    #[test]
    fn one_corner() {
        const TEST: &str = "\
#######
#S....#
#####.#
#####.#
#####E#
#######
";
        assert_eq!(1007, part1(BufReader::new(TEST.as_bytes())).unwrap());
    }

    #[test]
    fn two_corners() {
        const TEST: &str = "\
#######
#S....#
#####.#
#####.#
#E....#
#######
";
        assert_eq!(2011, part1(BufReader::new(TEST.as_bytes())).unwrap());
    }

    #[test]
    fn two_corners_and_long_path() {
        const TEST: &str = "\
#######
#S....#
#.###.#
#.###.#
#E....#
#######
";
        assert_eq!(1003, part1(BufReader::new(TEST.as_bytes())).unwrap());
    }

    #[test]
    fn from_reddit() {
        const TEST: &str = "\
###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################
";
        assert_eq!(21148, part1(BufReader::new(TEST.as_bytes())).unwrap())
    }
}
