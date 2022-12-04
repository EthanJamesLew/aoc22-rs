// Day 2: Rock Paper Scissors
// Rock Paper Scissors
use aoc_rs::AoC;
use std::fs;

// rock paper scissors
mod rps {
    #[derive(Copy, Clone)]
    pub enum GameMove {
        Rock,
        Paper,
        Scissors,
    }

    #[derive(Copy, Clone)]
    pub enum GameOutcome {
        Lost,
        Draw,
        Won,
    }

    // map move to score
    pub fn get_move_score(m: &GameMove) -> u32 {
        match m {
            GameMove::Rock => 1,
            GameMove::Paper => 2,
            GameMove::Scissors => 3,
        }
    }

    // map game outcome to score
    pub fn get_outcome_score_char(o: &GameOutcome) -> u32 {
        match o {
            GameOutcome::Lost => 0,
            GameOutcome::Draw => 3,
            GameOutcome::Won => 6,
        }
    }

    // map character to opponent move
    pub fn get_opponent_move_char(c: &char) -> Option<GameMove> {
        match c {
            'A' => Some(GameMove::Rock),
            'B' => Some(GameMove::Paper),
            'C' => Some(GameMove::Scissors),
            _ => None,
        }
    }

    // map character to my move
    pub fn get_my_move_char(c: &char) -> Option<GameMove> {
        match c {
            'X' => Some(GameMove::Rock),
            'Y' => Some(GameMove::Paper),
            'Z' => Some(GameMove::Scissors),
            _ => None,
        }
    }

    // map character to outcome (part 2)
    pub fn get_outcome_char(o: &char) -> Option<GameOutcome> {
        match o {
            'X' => Some(GameOutcome::Lost),
            'Y' => Some(GameOutcome::Draw),
            'Z' => Some(GameOutcome::Won),
            _ => None,
        }
    }

    // play game (map moves to outcome)
    pub fn play_game(my_move: &GameMove, opponent_move: &GameMove) -> GameOutcome {
        return match my_move {
            GameMove::Rock => match opponent_move {
                GameMove::Rock => GameOutcome::Draw,
                GameMove::Paper => GameOutcome::Lost,
                GameMove::Scissors => GameOutcome::Won,
            },
            GameMove::Paper => match opponent_move {
                GameMove::Rock => GameOutcome::Won,
                GameMove::Paper => GameOutcome::Draw,
                GameMove::Scissors => GameOutcome::Lost,
            },
            GameMove::Scissors => match opponent_move {
                GameMove::Rock => GameOutcome::Lost,
                GameMove::Paper => GameOutcome::Won,
                GameMove::Scissors => GameOutcome::Draw,
            },
        };
    }

    // given outcome and opponent move, get my move (part 2)
    pub fn get_my_move(outcome: &GameOutcome, opponent_move: &GameMove) -> GameMove {
        return match outcome {
            GameOutcome::Draw => match opponent_move {
                GameMove::Rock => GameMove::Rock,
                GameMove::Paper => GameMove::Paper,
                GameMove::Scissors => GameMove::Scissors,
            },
            GameOutcome::Lost => match opponent_move {
                GameMove::Rock => GameMove::Scissors,
                GameMove::Paper => GameMove::Rock,
                GameMove::Scissors => GameMove::Paper,
            },
            GameOutcome::Won => match opponent_move {
                GameMove::Rock => GameMove::Paper,
                GameMove::Paper => GameMove::Scissors,
                GameMove::Scissors => GameMove::Rock,
            },
        };
    }

    // calculate total score
    pub fn score_game(my_move: &GameMove, opponent_move: &GameMove) -> u32 {
        get_move_score(my_move) + get_outcome_score_char(&play_game(my_move, opponent_move))
    }

}

struct Day02 {
    games: Vec<(rps::GameMove, rps::GameMove)>,
    games_outcome: Vec<(rps::GameOutcome, rps::GameMove)>,
}

impl AoC for Day02 {
    type PuzzleReturnType = u32;

    fn day() -> u32 {
        2
    }

    fn from_file(filename: &str) -> Option<Self> {
        let mut day02 = Day02 {
            games: Vec::<(rps::GameMove, rps::GameMove)>::new(),
            games_outcome:
                Vec::<(rps::GameOutcome, rps::GameMove)>::new(),
        };
        let f_string = fs::read_to_string(filename).unwrap();
        for line in f_string.split('\n') {
            let op_move =
                rps::get_opponent_move_char(&line.chars().nth(0).unwrap()).unwrap();
            let my_move =
                rps::get_my_move_char(&line.chars().nth(2).unwrap()).unwrap();
            let game_outcome =
                rps::get_outcome_char(&line.chars().nth(2).unwrap()).unwrap();
            day02.games.push((my_move, op_move));
            day02.games_outcome.push((game_outcome, op_move));
        }
        Some(day02)
    }

    fn part1(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Ok(self
            .games
            .iter()
            .map(|g| rps::score_game(&g.0, &g.1))
            .sum())
    }

    fn part2(&self) -> Result<Self::PuzzleReturnType, &'static str> {
        Ok(self
            .games_outcome
            .iter()
            .map(|g| {
                rps::score_game(&rps::get_my_move(&g.0, &g.1), &g.1)
            })
            .sum())
    }
}

fn main() {
    Day02::from_file("./inputs/day02/input.txt").unwrap().run();
}

#[cfg(test)]
mod tests {
    use crate::{Day02, rps};
    use aoc_rs::AoC;
    static TEST_FNAME: &str = "./inputs/day02/test.txt";
    
    fn score_char_game(my_move: &char, opponent_move: &char) -> u32 {
        rps::score_game(
            &rps::get_my_move_char(my_move).unwrap(),
            &rps::get_opponent_move_char(opponent_move).unwrap(),
        )
    }

    #[test]
    fn test_rps() {
        assert_eq!(score_char_game(&'Y', &'A'), 8);
        assert_eq!(score_char_game(&'X', &'B'), 1);
        assert_eq!(score_char_game(&'Z', &'C'), 6);
    }

    #[test]
    fn test_part1() {
        assert_eq!(Day02::from_file(TEST_FNAME).unwrap().part1().unwrap(), 15);
    }

    #[test]
    fn test_part2() {
        assert_eq!(Day02::from_file(TEST_FNAME).unwrap().part2().unwrap(), 12);
    }
}
