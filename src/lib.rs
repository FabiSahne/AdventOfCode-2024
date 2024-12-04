use std::io::BufRead;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
