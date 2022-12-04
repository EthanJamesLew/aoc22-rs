# Advent of Code 2022
![Rust CI/CD](https://github.com/EthanJamesLew/aoc22-rs/actions/workflows/rust.yml/badge.svg)
![Profiling](https://github.com/EthanJamesLew/aoc22-rs/actions/workflows/performance.yml/badge.svg)

The solutions are written as examples. They can be run with
```shell
cargo run --example day01
```
and tested with
```shell
cargo test --example day01
```

## User Guide
They are implemented by implementing the `AoC` trait
```rust
use aoc_rs::AoC;

struct Day02 {
    // add models from input here
}

impl AoC for Day02 {
    // solution return type
    type PuzzleReturnType = u64;

    // set the day here
    fn day() -> u32 {
        2
    }

    // implement your file loader here
    fn from_file(filename: &str) -> Option<Self> {
        Some(Day02 {})
    }

    // implement the part1 solution here
    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Err("not implemented")
    }

    // implemented the part2 solution here
    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Err("not implemented")
    }
}

fn main() {
    // run everything
    Day02::from_file("./inputs/day02/input.txt").unwrap().run();
}

// set up the tests here
#[cfg(test)]
mod tests {
    use crate::Day02;
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day02/test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(
            Day02::from_file(TEST_FNAME).unwrap().part1().unwrap(),
            24_000
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day02::from_file(TEST_FNAME).unwrap().part2().unwrap(),
            45_000
        );
    }
}

```

## Compare Against Python

For perf, run
```shell
sudo sh -c 'echo 1 >/proc/sys/kernel/perf_event_paranoid'
```

For Day xx, run
```shell
./profile/compare_python.sh xx
```
