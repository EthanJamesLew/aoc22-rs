use aoc_rs::AoC;

struct Day02 {}

impl AoC for Day02 {
    type PuzzleReturnType = u64;

    fn day() -> u32 {
        2
    }

    fn from_file(filename: &str) -> Option<Self> {
        Some(Day02 {})
    }

    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Err("not implemented")
    }

    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Err("not implemented")
    }
}

fn main() {
    Day02::from_file("./inputs/day02/input.txt").unwrap().run();
}

#[cfg(test)]
mod tests {
    use crate::Day02;
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day01/test.txt";

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
