use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "15";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST_SMALL: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

const TEST_LARGE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

fn move_possible(map: &[Vec<char>], dir: u8, mut position: Vec2<usize>) -> Option<Vec2<usize>> {
    while position.x > 0
        && position.y > 0
        && position.y < map.len() - 1
        && position.x < map[0].len() - 1
    {
        match dir {
            b'>' => position.x += 1,
            b'v' => position.y += 1,
            b'<' => position.x -= 1,
            b'^' => position.y -= 1,
            _ => unreachable!(),
        }
        if map[position.y][position.x] == '.' {
            return Some(position);
        } else if map[position.y][position.x] == '#' {
            return None;
        }
    }
    None
}

fn move_robot(map: &mut [Vec<char>], dir: u8, mut position: Vec2<usize>) -> Option<Vec2<usize>> {
    loop {
        let next_pos = match dir {
            b'>' => Vec2 {
                x: position.x - 1,
                y: position.y,
            },
            b'v' => Vec2 {
                x: position.x,
                y: position.y - 1,
            },
            b'<' => Vec2 {
                x: position.x + 1,
                y: position.y,
            },
            b'^' => Vec2 {
                x: position.x,
                y: position.y + 1,
            },
            _ => unreachable!(),
        };
        match map[next_pos.y][next_pos.x] {
            '.' | '#' => return None,
            'O' => {
                let tmp = map[next_pos.y][next_pos.x];
                map[next_pos.y][next_pos.x] = map[position.y][position.x];
                map[position.y][position.x] = tmp;
                position = next_pos;
            }
            '@' => {
                let tmp = map[next_pos.y][next_pos.x];
                map[next_pos.y][next_pos.x] = map[position.y][position.x];
                map[position.y][position.x] = tmp;
                return Some(position);
            }
            _ => unreachable!(),
        }
    }
}

fn sum_of_gps_coords(map: Vec<Vec<char>>) -> usize {
    let mut sum = 0;
    for y in 0..map.len() {
        for x in 0..map[0].len() {
            if map[y][x] == 'O' {
                sum += 100 * y + x;
            }
        }
    }
    sum
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(mut reader: R) -> Result<usize> {
        let mut input = String::new();
        reader.read_to_string(&mut input)?;
        let (map_str, moves) = input.split("\n\n").next_tuple().unwrap();
        let mut map = read_lines_to_vec_vec_char(BufReader::new(map_str.as_bytes()));
        let mut robot_position = Vec2::from(
            map.iter()
                .enumerate()
                .filter(|(_, l)| l.contains(&'@'))
                .map(|(y, l)| {
                    let x = l.iter().position(|&c| c == '@').unwrap();
                    (x, y)
                })
                .next()
                .unwrap(),
        );

        // println!("Initial state:");
        // print_map(&map);

        for direction in moves.bytes() {
            if [b'^', b'>', b'<', b'v'].contains(&direction) {
                if let Some(free_pos) = move_possible(&map, direction, robot_position) {
                    if let Some(pos) = move_robot(&mut map, direction, free_pos) {
                        robot_position = pos;
                    }
                }
                // println!("Move {}:", direction as char);
                // print_map(&map);
            }
        }

        let answer = sum_of_gps_coords(map);
        Ok(answer)
    }

    // Tests
    assert_eq!(2028, part1(BufReader::new(TEST_SMALL.as_bytes()))?);
    assert_eq!(10092, part1(BufReader::new(TEST_LARGE.as_bytes()))?);

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
