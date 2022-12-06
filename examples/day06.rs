#![feature(iter_array_chunks)]
// Day 6: Tuning Trouble
use aoc_rs::AoC;
use std::fs;

mod dev {
    // windowing scan
    pub fn scan_overlap(window_size: usize, inp: &String) -> usize{
        for idx in window_size..inp.len() {
            let chunk = &inp[idx-window_size..idx].to_string();
            let mut any_equal = false;
            for (i, ci) in chunk.chars().enumerate() {
                for (j, cj) in chunk.chars().enumerate() {
                    if (i != j) && (ci == cj) {
                        any_equal = true;
                    }
                }
            }
            if !any_equal {
                return idx as usize;
            } 
        }
        0
    }
}

struct Day06 {
    line: String
}

impl AoC for Day06 {
    type PuzzleReturnType = usize;

    fn day() -> u32 {
        6
    }

    fn from_file(filename: &str) -> Option<Self> {
        let f_str = fs::read_to_string(filename).unwrap();
        Some(
            Self {
                line: f_str
            }
        )
    }

    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Ok(dev::scan_overlap(4, &self.line))
    }

    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Ok(dev::scan_overlap(14, &self.line))
    }
}

fn main() {
    // run everything
    Day06::from_argparse("./inputs/day06/input.txt").unwrap().run();
}

// set up the tests here
#[cfg(test)]
mod tests {
    use crate::{Day06, dev};
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day06/test.txt";

    #[test]
    fn test_cases(){
        assert_eq!(
            dev::scan_overlap(4, &format!("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            5
        );
        assert_eq!(
            dev::scan_overlap(4, &format!("nppdvjthqldpwncqszvftbrmjlhg")),
            6
        );
        assert_eq!(
            dev::scan_overlap(4, &format!("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            10
        );
        assert_eq!(
            dev::scan_overlap(4, &format!("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            11
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Day06::from_file(TEST_FNAME).unwrap().part1().unwrap(),
            7 
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day06::from_file(TEST_FNAME).unwrap().part2().unwrap(),
            19
        );
    }
}
