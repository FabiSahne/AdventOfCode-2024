use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
type Data = (Vec<Vec<usize>>, Vec<Vec<usize>>);

fn read_data<R: BufRead>(reader: R) -> Result<Data> {
    let mut ordering = vec![];
    let mut updates = vec![];
    let mut read_ordering = false;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            read_ordering = true;
            continue;
        }
        if read_ordering {
            updates.push(
                line.split(',')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            );
        } else {
            ordering.push(
                line.split('|')
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            );
        }
    }

    Ok((ordering, updates))
}

fn fix_order(a: &usize, b: &usize, ordering: &[Vec<usize>]) -> Ordering {
    if ordering.contains(&vec![*a, *b]) {
        return Ordering::Less;
    }
    if ordering.contains(&vec![*b, *a]) {
        return Ordering::Greater;
    }
    Ordering::Equal
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (ordering, updates) = read_data(reader)?;

        let mut answer = 0;

        for update in updates {
            let mut order_is_correct = true;
            for order in &ordering {
                if let Some(first) = update.iter().position(|&item| item == order[0]) {
                    if let Some(second) = update.iter().position(|&item| item == order[1]) {
                        if first > second {
                            order_is_correct = false;
                            break;
                        }
                    }
                }
            }
            if order_is_correct {
                answer += update[update.len() / 2];
            }
        }

        Ok(answer)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {result}");
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (ordering, mut updates) = read_data(reader)?;

        let mut answer = 0;

        for update in &mut updates {
            let mut order_was_incorrect = false;
            for order in &ordering {
                if let Some(first) = update.iter().position(|&item| item == order[0]) {
                    if let Some(second) = update.iter().position(|&item| item == order[1]) {
                        if first > second {
                            order_was_incorrect = true;
                        }
                    }
                }
            }
            if order_was_incorrect {
                update.sort_unstable_by(|a, b| fix_order(a, b, &ordering));
                answer += update[update.len() / 2];
            }
        }

        Ok(answer)
    }

    assert_eq!(123, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {result}");
    //endregion

    Ok(())
}
