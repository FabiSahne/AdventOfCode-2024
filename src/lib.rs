use crate::Direction::*;
use itertools::Itertools;
use std::io::BufRead;
use std::ops::{Add, Sub};
use std::str::FromStr;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {day:0>2}");
}

// Additional common functions

/// Reads Lines from `reader` into Vector containing Vectors of line contents split by whitespace
/// and parsed to T
///
/// # Example
///
///```
/// use std::io::BufReader;
/// use adv_code_2024::read_lines_to_vec_vec_parsed;
/// let before = "\
/// 1 2 3
/// 4 5 6";
/// let after = vec![
///     vec![1, 2, 3],
///     vec![4, 5, 6],
/// ];
/// assert_eq!(after, read_lines_to_vec_vec_parsed(BufReader::new(before.as_bytes())))
/// ```
/// # Panics
///
/// This function panics if the read bytes are not valid UTF-8.
pub fn read_lines_to_vec_vec_parsed<R, T>(reader: R) -> Vec<Vec<T>>
where
    R: BufRead,
    T: FromStr + Default,
{
    reader
        .lines()
        .map(|l| {
            l.expect("`reader` should contain text")
                .split_whitespace()
                .map(|num| num.parse::<T>().unwrap_or_default())
                .collect::<Vec<T>>()
        })
        .collect::<Vec<Vec<T>>>()
}

/// Reads contents of `reader` into `Vec<Vec<char>>`. The outer Vector contains the lines,
/// the inner Vector contains the singular `char`s from the `reader` contents.
///
/// # Example
/// ```
/// use std::io::BufReader;
/// use adv_code_2024::read_lines_to_vec_vec_char;
/// let before = "\
/// ABC
/// DEF
/// ";
/// let after = vec![
///     vec!['A','B','C'],
///     vec!['D','E','F']
/// ];
/// assert_eq!(after, read_lines_to_vec_vec_char(BufReader::new(before.as_bytes())))
/// ```
///
/// # Panics
///
/// Panics if the read bytes do not contain valid UTF-8.
pub fn read_lines_to_vec_vec_char<R: BufRead>(reader: R) -> Vec<Vec<char>> {
    reader
        .lines()
        .map(|l| {
            l.expect("`reader` should contain text")
                .chars()
                .collect::<Vec<char>>()
        })
        .collect::<Vec<Vec<char>>>()
}

pub fn get_start_and_goal_from_grid(grid: &[Vec<char>]) -> (Vec2<usize>, Vec2<usize>) {
    ['S', 'E']
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
        .unwrap()
}

#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

macro_rules! __turn {
    ($__rhs:ident, $dir_left:ident, $dir_straight:ident, $dir_right:ident) => {
        if $__rhs == 0 {
            $dir_straight
        } else if $__rhs.is_negative() {
            $dir_left + ($__rhs + 1)
        } else {
            $dir_right + ($__rhs - 1)
        }
    };
}

impl Add<i8> for Direction {
    type Output = Direction;

    fn add(self, rhs: i8) -> Self::Output {
        match self {
            North => {
                __turn!(rhs, West, North, East)
            }
            East => {
                __turn!(rhs, North, East, South)
            }
            South => {
                __turn!(rhs, East, South, West)
            }
            West => {
                __turn!(rhs, South, West, North)
            }
        }
    }
}

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(tuple: (T, T)) -> Self {
        Vec2 {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> Add<Direction> for Vec2<T>
where
    T: From<u8> + Sub<Output = T> + Add<Output = T>,
{
    type Output = Vec2<T>;

    fn add(self, rhs: Direction) -> Self::Output {
        let one: T = 1u8.into();
        match rhs {
            North => Vec2 {
                x: self.x,
                y: self.y - one,
            },
            East => Vec2 {
                x: self.x + one,
                y: self.y,
            },
            South => Vec2 {
                x: self.x,
                y: self.y + one,
            },
            West => Vec2 {
                x: self.x - one,
                y: self.y,
            },
        }
    }
}

#[allow(dead_code)]
pub fn print_vec_vec_char(map: &[Vec<char>]) {
    for row in map {
        println!("{}", row.iter().collect::<String>());
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
