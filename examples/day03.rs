// Day 3: Rucksack Reorganization
#![feature(iter_array_chunks)]
use aoc_rs::AoC;
use std::fs;

// rucksack methods
mod rsack {
    // one hot encode [A-Za-z] ascii
    // offset by A from ASCII table for the puzzle
    pub fn encode(c: char) -> u64 {
        let ascii_uint = c as u64;
        let bit_pos = ascii_uint - ('A' as u64);
        return 1 << bit_pos
    }

    // encode entire string with one hot
    // bitwise-or every character encoding
    pub fn encode_str(s: &str) -> u64 {
        let mut res = 0;
        for c in s.chars(){
            res |= encode(c);
        }
        res
    }

    // get score from character
    pub fn score(c: char) -> u64 {
        let c_ascii = c as u8;
        if c_ascii >= ('a' as u8) {
            return (c_ascii - ('a' as u8) + 1) as u64;
        } else {
            return (c_ascii - ('A' as u8) + 27) as u64;
        }
    }

    // find one character of intersection between strings
    // bitwise-and both string encodings 
    pub fn intersection_example(s0: &str, s1: &str) -> Option<char> {
        decode(
            encode_str(s0) & encode_str(s1)
        )
    }
    
    // find one character of intersection between 3 strings (part2)
    // bitwise-and both string encodings 
    pub fn intersection_3_example(s0: &str, s1: &str, s2: &str) -> Option<char> {
        decode(
            encode_str(s0) & encode_str(s1) & encode_str(s2)
        )
    }

    // decode frpm one hot
    // offset by A from ASCII table for the puzzle
    pub fn decode(u: u64) -> Option<char> {
        for bp in 0..64 {
            if (u & (1 << bp)) != 0 {
                let ascii_uint = (bp + ('A' as u64)) as u8;
                return Some(ascii_uint as char);
            }
        }
        return None;
    }
}

struct Day03 {
    rucksacks: String 
}

impl AoC for Day03 {
    // solution return type
    type PuzzleReturnType = u64;

    // set the day here
    fn day() -> u32 {
        3
    }

    // implement your file loader here
    fn from_file(filename: &str) -> Option<Self> {
       let file_str = fs::read_to_string(filename).unwrap();
       Some(
        Day03 {
            rucksacks: file_str
        }
       )
    }

    // implement the part1 solution here
    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        let mut total_score: u64 = 0;
        for line in self.rucksacks.split('\n') {
            let (comp_a, comp_b) = line.split_at(line.len() / 2);
            let common = rsack::intersection_example(comp_a, comp_b);
            match common {
                Some(s) => {
                    total_score += rsack::score(s);
                },
                None => {
                    println!("no common character found for {}", line);
                }
            }
        }
        Ok(total_score)
    }

    // implemented the part2 solution here
    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        let mut total_score: u64 = 0;
        for lines in self.rucksacks.split('\n').array_chunks::<3>() {
            let l1 = lines[0];
            let l2 = lines[1];
            let l3 = lines[2];
            let common = rsack::intersection_3_example(l1, l2, l3);
            match common {
                Some(s) => {
                    total_score += rsack::score(s);
                },
                None => {
                    println!("no common character found for {},{},{}", l1, l2, l3);
                }
            }
        }
        Ok(total_score)
    }
}

fn main() {
    // run everything
    Day03::from_file("./inputs/day03/input.txt").unwrap().run();
}

// set up the tests here
#[cfg(test)]
mod tests {
    use crate::{Day03, rsack};
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day03/test.txt";

    #[test]
    fn test_one_hot() {
        assert_eq!(rsack::encode('A'), 1);
        assert_eq!(rsack::encode('z'), 1 << 57);

        assert_eq!(rsack::decode(1).unwrap(), 'A');
        assert_eq!(rsack::decode(1 << 57).unwrap(), 'z');
    }

    #[test]
    fn test_intersection() {
        assert_eq!(rsack::intersection_example("vJrwpWtwJgWr", "hcsFMMfFFhFp").unwrap(), 'p');
        assert_eq!(rsack::intersection_example("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL").unwrap(), 'L');
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Day03::from_file(TEST_FNAME).unwrap().part1().unwrap(),
            157 
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day03::from_file(TEST_FNAME).unwrap().part2().unwrap(),
            70 
        );
    }
}
