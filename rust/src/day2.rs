use std::str::FromStr;

use crate::util::load;

#[derive(Debug, Copy, Clone)]
pub enum Choice {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Choice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Self::Rock),
            "B" | "Y" => Ok(Self::Paper),
            "C" | "Z" => Ok(Self::Scissors),
            _ => Err(()),
        }
    }
}

impl Choice {
    fn score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn wins_from(&self, other: &Self) -> bool {
        match self {
            Self::Rock => {
                if let Self::Scissors = other {
                    true
                } else {
                    false
                }
            }
            Self::Paper => {
                if let Self::Rock = other {
                    true
                } else {
                    false
                }
            }
            Self::Scissors => {
                if let Self::Paper = other {
                    true
                } else {
                    false
                }
            }
        }
    }

    fn expect_result(&self, expect: &GameResult) -> Self {
        match self {
            Self::Rock => match expect {
                GameResult::Lose => Self::Scissors,
                GameResult::Draw => Self::Rock,
                GameResult::Win => Self::Paper,
            },
            Self::Paper => match expect {
                GameResult::Lose => Self::Rock,
                GameResult::Draw => Self::Paper,
                GameResult::Win => Self::Scissors,
            },
            Self::Scissors => match expect {
                GameResult::Lose => Self::Paper,
                GameResult::Draw => Self::Scissors,
                GameResult::Win => Self::Rock,
            },
        }
    }
}

pub enum GameResult {
    Lose,
    Draw,
    Win,
}

impl FromStr for GameResult {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Draw),
            "Z" => Ok(Self::Win),
            _ => Err(()),
        }
    }
}

impl GameResult {
    fn score(&self) -> u32 {
        match self {
            Self::Lose => 0,
            Self::Draw => 3,
            Self::Win => 6,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Game1 {
    op: Choice,
    me: Choice,
}

impl FromStr for Game1 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choices: Vec<Choice> = s.split(' ').map(|c| c.parse().unwrap()).collect();
        Ok(Game1 {
            op: choices[0],
            me: choices[1],
        })
    }
}

impl Game1 {
    fn score(&self) -> u32 {
        let game_score = if self.me.wins_from(&self.op) {
            6
        } else if self.op.wins_from(&self.me) {
            0
        } else {
            3
        };
        game_score + self.me.score()
    }
}

pub struct Game2 {
    op: Choice,
    expect: GameResult,
}

impl FromStr for Game2 {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let choices: Vec<&str> = s.split(' ').collect();
        Ok(Game2 {
            op: choices[0].parse::<Choice>().unwrap(),
            expect: choices[1].parse::<GameResult>().unwrap(),
        })
    }
}

impl Game2 {
    fn score(&self) -> u32 {
        self.expect.score() + self.op.expect_result(&self.expect).score()
    }
}

pub fn part1() -> u32 {
    let games: Vec<Game1> = load("data/day2.txt");
    games.into_iter().map(|g| g.score()).sum()
}

pub fn part2() -> u32 {
    let games: Vec<Game2> = load("data/day2.txt");
    games.into_iter().map(|g| g.score()).sum()
}

#[cfg(test)]
mod tests {
    use crate::day2::{part1, part2};

    #[test]
    fn test_part1() {
        let score = part1();
        println!("Score is {}", score);
        assert_eq!(score, 11873);
    }

    #[test]
    fn test_part2() {
        let score = part2();
        println!("Score is {}", score);
        assert_eq!(part2(), 12014);
    }
}
