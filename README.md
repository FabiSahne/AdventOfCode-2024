# Advent of Code 2024 in Rust

The **Chief Historian** is always present for the big Christmas sleigh launch, but nobody has seen him in months! Last
anyone heard, he was visiting locations that are historically significant to the North Pole; a group of Senior
Historians has asked you to accompany them as they check the places they think he was most likely to visit.

As each location is checked, they will mark it on their list with a _star_. They figure the Chief Historian **must** be
in one
of the first fifty places they'll look, so in order to save Christmas, you need to help them get _fifty stars_ on their
list before Santa takes off on December 25th.

Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second
puzzle is unlocked when you complete the first. Each puzzle grants _one star_. Good luck!

## Day 01

You haven't even left yet and the group of Elvish Senior Historians has already hit a problem: their list of locations
to check is currently **empty**. Eventually, someone decides that the best place to check first would be the Chief
Historian's office.

Upon pouring into the office, everyone confirms that the Chief Historian is indeed nowhere to be found. Instead, the
Elves discover an assortment of notes and lists of historically significant locations! This seems to be the planning the
Chief Historian was doing before he left. Perhaps these notes can be used to determine which locations to search?

Throughout the Chief's office, the historically significant locations are listed not by name but by a unique number
called the **location ID**. To make sure they don't miss anything, The Historians split into two groups, each searching
the
office and trying to create their own complete list of location IDs.

There's just one problem: by holding the two lists up **side by side** (your puzzle input), it quickly becomes clear
that
the lists aren't very similar. Maybe you can help The Historians reconcile their lists?

For example:

```
3   4
4   3
2   5
1   3
3   9
3   3
```

Maybe the lists are only off by a small amount! To find out, pair up the numbers and measure how far apart they are.
Pair up the **smallest number in the left list** with the **smallest number in the right list**, then the
**second-smallest left number** with the **second-smallest right number**, and so on.

Within each pair, figure out **how far apart** the two numbers are; you'll need to **add up all of those distances**.
For example, if you pair up a `3` from the left list with a `7` from the right list, the distance apart is `4`;
if you pair up a `9` with a `3`, the distance apart is `6`.

In the example list above, the pairs and distances would be as follows:

- The smallest number in the left list is `1`, and the smallest number in the right list is `3`.
  The distance between them is `2`.
- The second-smallest number in the left list is `2`, and the second-smallest number in the right list is another `3`.
  The distance between them is `1`.
- The third-smallest number in both lists is `3`, so the distance between them is `0`.
- The next numbers to pair up are `3` and `4`, a distance of `1`.
- The fifth-smallest numbers in each list are `3` and `5`, a distance of `2`.
- Finally, the largest number in the left list is `4`, while the largest number in the right list is `9`; these are a
  distance `5` apart.

To find the **total distance** between the left list and the right list, add up the distances between all of the pairs
you
found. In the example above, this is `2 + 1 + 0 + 1 + 2 + 5`, a total distance of `11`!

Your actual left and right lists contain many location IDs. **What is the total distance between your lists?**

### Part 2

Your analysis only confirmed what everyone feared: the two lists of location IDs are indeed very different.

Or are they?

The Historians can't agree on which group made the mistakes or how to read most of the Chief's handwriting, but in the
commotion you notice an interesting detail: a lot of location IDs appear in both lists! Maybe the other numbers aren't
location IDs at all but rather misinterpreted handwriting.

This time, you'll need to figure out exactly how often each number from the left list appears in the right list.
Calculate a total **similarity score** by adding up each number in the left list after multiplying it by the number of
times
that number appears in the right list.

Here are the same example lists again:

```
3   4
4   3
2   5
1   3
3   9
3   3
```

For these example lists, here is the process of finding the similarity score:

- The first number in the left list is `3`. It appears in the right list three times,
  so the similarity score increases by `3 * 3 = 9`.
- The second number in the left list is `4`. It appears in the right list once,
  so the similarity score increases by `4 * 1 = 4`.
- The third number in the left list is `2`. It does not appear in the right list, so the similarity score does not
  increase (`2 * 0 = 0`).
- The fourth number, `1`, also does not appear in the right list.
- The fifth number, `3`, appears in the right list three times; the similarity score increases by `9`.
- The last number, `3`, appears in the right list three times; the similarity score again increases by `9`.

So, for these example lists, the similarity score at the end of this process is `31` (`9 + 4 + 0 + 0 + 9 + 9`).

Once again consider your left and right lists. **What is their similarity score?**

## Day 02

Fortunately, the first location The Historians want to search isn't a long walk from the Chief Historian's office.

While the [Red-Nosed Reindeer nuclear fusion/fission plant](https://adventofcode.com/2015/day/19) appears to contain no
sign of the Chief Historian, the engineers there run up to you as soon as they see you. Apparently, they **still** talk
about the time Rudolph was saved through molecular synthesis from a single electron.

They're quick to add that - since you're already here - they'd really appreciate your help analyzing some unusual data
from the Red-Nosed reactor. You turn to check if The Historians are waiting for you, but they seem to have already
divided into groups that are currently searching every corner of the facility. You offer to help with the unusual data.

The unusual data (your puzzle input) consists of many **reports**, one report per line. Each report is a list of numbers
called **levels** that are separated by spaces. For example:

```
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
```

This example data contains six reports each containing five levels.

The engineers are trying to figure out which reports are **safe**. The Red-Nosed reactor safety systems can only
tolerate levels that are either gradually increasing or gradually decreasing. So, a report only counts as safe if both
of the following are true:

- The levels are either **all increasing** or **all decreasing**.
- Any twp adjecent levels differ by **at least one** and **at most three**.

In the example above, the reportscan be found safe or unsafe by checking those rules:

- `7 6 4 2 1`: **Safe** because the levels are all decreasing by 1 or 2.
- `1 2 7 8 9`: **Unsafe** because `2 7` is an increase of 5.
- `9 7 6 2 1`: **Unsafe** because `6 2` is a decrease of 4.
- `1 3 2 4 5`: **Unsafe** because `1 3` is increasing, but `3 2` is decreasing.
- `8 6 4 4 1`: **Unsafe** because `4 4` is neither an increase or a decrease.
- `1 3 6 7 9`: **Safe** because the levels are all decreasing by 1, 2, or 3.

So, in this example, `2` reports are **safe**.

Analyze th unusual data from the emgineers. **How many reports are safe?**

### Part 2

The engineers are surprised by the low number of safe reports until they realize they forgot to tell you about the
Problem Dampener.

The Problem Dampener is a reactor-mounted module that lets the reactor safety systems **tolerate a single bad level** in
what would otherwise be a safe report. It's like the bad level never happened!

Now, the same rules apply as before, except if removing a single level from an unsafe report would make it safe, the
report instead counts as safe.

More of the above example's reports are now safe:

- `7 6 4 2 1`: **Safe** without removing any level.
- `1 2 7 8 9`: **Unsafe** regardless of which level is removed.
- `9 7 6 2 1`: **Unsafe** regardless of which level is removed.
- `1 3 2 4 5`: **Safe** by removing the second level, `3`.
- `8 6 4 4 1`: **Safe** by removing the third level, `4`.
- `1 3 6 7 9`: **Safe** without removing any level.

Thanks to the Problem Dampener, `4` reports are actually **safe**!

Update your analysis by handling situations where the Problem Dampener can remove a single level from unsafe reports.
**How many reports are now safe?**

---

## Calculation Times

|    Day | Part 1 | Part 2 |
|-------:|:-------|:-------|
| **01** | 213μs  | 178μs  |
| **02** | 299μs  | 372μs  |
| **03** |        |        |
| **04** |        |        |
| **05** |        |        |
| **06** |        |        |
| **07** |        |        |
| **08** |        |        |
| **09** |        |        |
| **10** |        |        |
| **11** |        |        |
| **12** |        |        |
| **13** |        |        |
| **14** |        |        |
| **15** |        |        |
| **16** |        |        |
| **17** |        |        |
| **18** |        |        |
| **19** |        |        |
| **20** |        |        |
| **21** |        |        |
| **22** |        |        |
| **23** |        |        |
| **24** |        |        |
| **25** |        |        |

---

## Template Usage

1. Whenever you're ready to start solving a new day's puzzle:
    - Open the `bin` folder, copy and paste the `NN.rs` file into it, and give it the corresponding name (`01.rs`,
      `02.rs`, etc.).
    - In the `input` folder, create and fill the input data file (`01.txt`, `02.txt`, etc.).
    - Fill in the `DAY` constant in the freshly created file.
    - Run the current day's solution to check if it compiles (you can use the gutter icon next to the `main` function).
    - Fill in `<TEST-INPUT>`.
    - Write the expected answer for the test data in the `assert_eq` statement in *Part 1*.
    - Now you're ready to write your solution in the `part1` function (inside `main`).
    - Use `Shift+F10` (Win/Linux) or `Ctrl-R` (macOS) to re-run the same program.

2. When you're done with the first part of the puzzle, use folding to hide *Part 1*.

3. Uncomment *Part 2*, fill in the test data assertion, and start solving it.
