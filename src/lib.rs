use anyhow::Result;
use std::io::BufRead;
use std::str::FromStr;

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {day:0>2}");
}

// Additional common functions

/// Reads Lines from reader into Vector containing Vectors of line contents split by whitespace
///
/// # Errors
///
/// This function has the same error semantics as [`BufRead::read_until`] and will also return an error if the
/// read bytes are not valid UTF-8. If an I/ O error is encountered then buf may contain some bytes
/// already read in the event that all data read so far was valid UTF-8.
pub fn read_lines_to_vec<R: BufRead, T>(reader: R) -> Result<Vec<Vec<T>>>
where
    T: FromStr + Default,
{
    let mut vec = vec![];
    for rline in reader.lines() {
        let line = rline?;
        vec.push(
            line.split_whitespace()
                .map(|s| s.parse::<T>().unwrap_or_default())
                .collect::<Vec<T>>(),
        );
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
