// Day 1: Calorie Counting
use aoc_rs::AoC;
use std::fs;
use std::vec::Vec;

// Elf model
struct Elf {
    calories: Vec<u64>,
}

impl Elf {
    fn new() -> Self {
        Elf {
            // TODO: maybe an array isn't the best representation?
            calories: Vec::<u64>::new(),
        }
    }

    // push calories
    fn push_calories(&mut self, calories: u64) {
        self.calories.push(calories);
    }

    // get total calories from each elf
    fn total_calories(&self) -> u64 {
        self.calories.iter().sum()
    }

    // read a list of elfs from the puzzle input file
    fn read_elfs(filepath: &str) -> Vec<Elf> {
        let mut elfs = Vec::<Elf>::new();

        let filestr = fs::read_to_string(filepath).unwrap();

        for s in filestr.split("\n\n") {
            let mut e = Elf::new();
            for val in s.split('\n') {
                let calories: u64 = val.to_string().parse().unwrap();
                e.push_calories(calories);
            }
            elfs.push(e);
        }
        elfs
    }
}

// implement AoC trait for day 1
struct Day01 {
    elfs: Vec<Elf>,
}

impl AoC for Day01 {
    // we return total calories (unsigned int)
    type PuzzleReturnType = u64;

    // input parser
    fn from_file(filename: &str) -> Option<Self> {
        Some(Self {
            elfs: Elf::read_elfs(filename),
        })
    }

    // day 1 puzzle
    fn day() -> u32 {
        1
    }

    // top elf total calories
    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        let total_calories: Vec<u64> = self.elfs.iter().map(|e| e.total_calories()).collect();
        Ok(*total_calories.iter().max().unwrap())
    }

    // top 3 elf total calories summed together
    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        let mut total_calories: Vec<u64> = self.elfs.iter().map(|e| e.total_calories()).collect();
        total_calories.sort();
        Ok(
            total_calories[total_calories.len() - 3..total_calories.len()]
                .iter()
                .sum::<u64>(),
        )
    }
}

fn main() {
    Day01::from_file("./inputs/day01/input.txt").unwrap().run();
}

#[cfg(test)]
mod tests {
    use crate::Day01;
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day01/test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(
            Day01::from_file(TEST_FNAME).unwrap().part1().unwrap(),
            24_000
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day01::from_file(TEST_FNAME).unwrap().part2().unwrap(),
            45_000
        );
    }
}
