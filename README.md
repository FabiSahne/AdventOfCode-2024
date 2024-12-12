# Advent of Code 2024 in Rust

## Calculation Times

Average over five runs:

|    Day | Part 1 | Part 2  |
|-------:|:-------|:--------|
| **01** | 254μs  | 225μs   |
| **02** | 375μs  | 412μs   |
| **03** | 311μs  | 344μs   |
| **04** | 434μs  | 211μs   |
| **05** | 1.82ms | 5.00ms  |
| **06** | 550μs  | 5.486s  |
| **07** | 6.17ms | 156.1ms |
| **08** | 117μs  | 112μs   |
| **09** | 2.25ms | _DNF_   |
| **10** | _DNF_  | _DNF_   |
| **11** | 3.58ms | 7.00ms  |
| **12** | 4.43ms |         |
| **13** |        |         |
| **14** |        |         |
| **15** |        |         |
| **16** |        |         |
| **17** |        |         |
| **18** |        |         |
| **19** |        |         |
| **20** |        |         |
| **21** |        |         |
| **22** |        |         |
| **23** |        |         |
| **24** |        |         |
| **25** |        |         |

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
