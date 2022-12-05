use clap::{command, Arg};
use colored::Colorize;
use std::fmt::Display;

pub trait AoC: Sized {
    type PuzzleReturnType: Display;

    // all puzzles must be loadable from file
    fn from_file(filename: &str) -> Option<Self>;

    // for making a AoC CLI app
    fn from_argparse(default_filename: &str) -> Option<Self> {
        let matches = command!()
            .name("Advent of Code Solutions")
            .version("0.1.0")
            .author("Ethan Lew")
            .about("Produces Solutions to AoC")
            .arg(
                Arg::new("input_filepath")
                    .short('f')
                    .long("file")
                    .help("Advent of Code Puzzle Input File"),
            )
            .get_matches();
        let inp_string = default_filename.to_string();
        let input_file = matches
            .get_one::<String>("input_filepath")
            .unwrap_or(&inp_string);
        return Self::from_file(input_file.as_str());
    }

    // puzzle have a day
    fn day() -> u32;

    // puzzles have a part 1
    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Err("part1 not implemented")
    }

    // puzzles have a part 2
    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Err("part2 not implemented")
    }

    fn run_part(part_num: usize, part_result: Result<Self::PuzzleReturnType, &'static str>) {
        match part_result {
            Err(s) => {
                let err_str = format!("Error Encountered in Part {}: <{}>", part_num, s).red();
                println!(
                    "--Part {}--\n\t{}\n--End Part {}--",
                    part_num, err_str, part_num
                )
            }
            Ok(s) => {
                let sol_str = format!("Solution to Part {}: {}", part_num, s).green();
                println!(
                    "--Part {}--\n\t{}\n--End Part {}--",
                    part_num, sol_str, part_num
                )
            }
        }
    }

    fn run(&self) {
        let day_str = format!("Advent of Code (Day {})", Self::day()).blue();
        println!("{}", day_str);
        Self::run_part(1, Self::part1(&self));
        Self::run_part(2, Self::part2(&self));
        println!("{}", "Done!".blue())
    }
}
