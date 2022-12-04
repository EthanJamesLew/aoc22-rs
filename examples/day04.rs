// Day 4: Camp Cleanup
use aoc_rs::AoC;
use std::fs;

// put these in trait in case part 2 has something different
trait GeometricDomain<T> {
    // whether self \subseteq d
    fn is_subseteq(&self, d: &T) -> bool;

    // whether self \intersects d \ne nullset
    fn is_overlap(&self, d: &T) -> bool;
}

// implement as line segments
#[derive(Copy, Clone)]
struct LineSegment {
    end_pts: (u64, u64),
}

impl LineSegment {
    fn new(start: u64, end: u64) -> Self {
        Self {
            end_pts: (start, end),
        }
    }
}

impl GeometricDomain<LineSegment> for LineSegment {
    fn is_subseteq(&self, d: &LineSegment) -> bool {
        self.end_pts.0 >= d.end_pts.0 && self.end_pts.1 <= d.end_pts.1
    }

    fn is_overlap(&self, d: &LineSegment) -> bool {
        (self.end_pts.0 <= d.end_pts.1 && self.end_pts.1 >= d.end_pts.0)
            || (d.end_pts.0 <= self.end_pts.1 && d.end_pts.1 >= self.end_pts.0)
    }
}

struct Day04 {
    segments: Vec<(LineSegment, LineSegment)>,
}

impl AoC for Day04 {
    type PuzzleReturnType = u64;

    fn day() -> u32 {
        4
    }

    // TODO: this parser is ugly
    fn from_file(filename: &str) -> Option<Self> {
        let mut day04 = Self {
            segments: Vec::<(LineSegment, LineSegment)>::new(),
        };
        let file_str = fs::read_to_string(filename).unwrap();
        for line in file_str.split("\n") {
            // TODO: stop using split and nth?
            let seg0 = line.split(",").nth(0).unwrap();
            let seg1 = line.split(",").nth(1).unwrap();
            let l0 = LineSegment::new(
                seg0.split("-").nth(0).unwrap().parse::<u64>().unwrap(),
                seg0.split("-").nth(1).unwrap().parse::<u64>().unwrap(),
            );
            let l1 = LineSegment::new(
                seg1.split("-").nth(0).unwrap().parse::<u64>().unwrap(),
                seg1.split("-").nth(1).unwrap().parse::<u64>().unwrap(),
            );
            day04.segments.push((l0, l1));
        }
        Some(day04)
    }

    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        let overlaps: Vec<&(LineSegment, LineSegment)> = self
            .segments
            .iter()
            .filter(|(s0, s1)| s0.is_subseteq(s1) || s1.is_subseteq(s0))
            .collect();
        Ok(overlaps.len() as u64)
    }

    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        let overlaps: Vec<&(LineSegment, LineSegment)> = self
            .segments
            .iter()
            .filter(|(s0, s1)| s0.is_overlap(s1))
            .collect();
        Ok(overlaps.len() as u64)
    }
}

fn main() {
    // run everything
    Day04::from_file("./inputs/day04/input.txt").unwrap().run();
}

// set up the tests here
#[cfg(test)]
mod tests {
    use crate::Day04;
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day04/test.txt";

    #[test]
    fn test_part1() {
        assert_eq!(Day04::from_file(TEST_FNAME).unwrap().part1().unwrap(), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day04::from_file(TEST_FNAME).unwrap().part2().unwrap(), 4);
    }
}
