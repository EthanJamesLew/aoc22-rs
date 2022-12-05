use aoc_rs::AoC;
use std::fs;

mod cs {
    pub fn move_crates(
        scenario: &mut Vec<Vec<char>>,
        quantity: usize,
        from_loc: usize,
        to_loc: usize,
    ) -> Vec<Vec<char>> {
        let mut updated_scenario = scenario.clone();
        let from_stack = &scenario[from_loc - 1];

        let moved = &from_stack[(from_stack.len() - quantity)..from_stack.len()];
        for m in moved.iter().rev() {
            updated_scenario[to_loc - 1].push(*m);
            updated_scenario[from_loc - 1].pop();
        }
        updated_scenario
    }

    pub fn move_crates_9001(
        scenario: &mut Vec<Vec<char>>,
        quantity: usize,
        from_loc: usize,
        to_loc: usize,
    ) -> Vec<Vec<char>> {
        let mut updated_scenario = scenario.clone();
        let from_stack = &scenario[from_loc - 1];

        let moved = &from_stack[(from_stack.len() - quantity)..from_stack.len()];
        for m in moved.iter() {
            updated_scenario[to_loc - 1].push(*m);
            updated_scenario[from_loc - 1].pop();
        }
        updated_scenario
    }

    pub fn get_message(scenario: &Vec<Vec<char>>) -> String {
        let message: String = scenario
            .iter()
            .map(|stack| stack.last().unwrap_or(&' '))
            .collect();
        message
    }
}

struct Day05 {
    // TODO: vecs were not a good choice here
    stacks: Vec<Vec<char>>,
    moves: Vec<(usize, usize, usize)>,
}

impl Day05 {
    fn play_game(&self) -> String {
        let mut scen = self.stacks.clone();
        for (q, f, t) in &self.moves {
            scen = cs::move_crates(&mut scen, *q, *f, *t);
        }
        cs::get_message(&scen)
    }

    fn play_game_9001(&self) -> String {
        let mut scen = self.stacks.clone();
        for (q, f, t) in &self.moves {
            scen = cs::move_crates_9001(&mut scen, *q, *f, *t);
        }
        cs::get_message(&scen)
    }
}

impl AoC for Day05 {
    type PuzzleReturnType = String;

    fn day() -> u32 {
        5
    }

    // implement your file loader here
    fn from_file(filename: &str) -> Option<Self> {
        let file_str = fs::read_to_string(filename).ok().unwrap();
        let mut parts = file_str.split("\n\n");
        let layout = parts.nth(0).unwrap();
        let commands = parts.nth(0).unwrap();

        let mut layout_rev = layout.split('\n').rev();
        let nums = layout_rev.nth(0).unwrap().trim();
        let max_num: usize = nums.split(' ').last().unwrap().parse().unwrap();

        let mut scenario: Vec<Vec<char>> = vec![];
        for _ in 0..max_num {
            let vi: Vec<char> = vec![];
            scenario.push(vi);
        }

        for line in layout_rev {
            for idx in 0..max_num {
                let bx = line.chars().nth(idx * 4 + 1).unwrap();
                if bx != ' ' {
                    scenario[idx].push(bx);
                }
            }
        }

        let mut command_v: Vec<(usize, usize, usize)> = vec![];
        for line in commands.split('\n') {
            let mut comm = line.clone().split(" ");
            let q: usize = comm.nth(1).unwrap().to_string().parse().unwrap();
            let f: usize = comm.nth(1).unwrap().to_string().parse().unwrap();
            let t: usize = comm.nth(1).unwrap().to_string().parse().unwrap();
            command_v.push((q, f, t))
        }
        Some(Day05 {
            stacks: scenario,
            moves: command_v,
        })
    }

    // implement the part1 solution here
    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Ok(self.play_game())
    }

    // implemented the part2 solution here
    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Ok(self.play_game_9001())
    }
}

fn main() {
    // run everything
    Day05::from_file("./inputs/day05/input.txt").unwrap().run();
}

// set up the tests here
#[cfg(test)]
mod tests {
    use crate::cs;
    use crate::Day05;
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day05/test.txt";

    #[test]
    fn test_scenario() {
        let mut scenario = vec![vec!['z', 'n'], vec!['m', 'c', 'd'], vec!['p']];
        scenario = cs::move_crates(&mut scenario, 1, 2, 1);
        scenario = cs::move_crates(&mut scenario, 3, 1, 3);
        scenario = cs::move_crates(&mut scenario, 2, 2, 1);
        scenario = cs::move_crates(&mut scenario, 1, 1, 2);
        assert_eq!(cs::get_message(&scenario), format!("cmz"));
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            Day05::from_file(TEST_FNAME).unwrap().part1().unwrap(),
            format!("CMZ")
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            Day05::from_file(TEST_FNAME).unwrap().part2().unwrap(),
            format!("MCD")
        );
    }
}
